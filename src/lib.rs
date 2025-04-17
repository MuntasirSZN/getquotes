pub mod background;
pub mod cache;
pub mod cli;
pub mod config;
pub mod logger;
pub mod quotes;
pub mod types;

use crate::cli::Args;
use crate::config::Config;
use crate::config::{load_or_create_config, load_or_create_config_from_path, parse_hex_color};
use clap::CommandFactory;
use clap_complete::generate;
use colored::*;
use git_rev::try_revision_string;
use log::{debug, error, info, warn};
use rand::{Rng, rng as thread_rng};
use reqwest::Client;
use std::error::Error as StdError;
use std::io;

pub async fn run(args: Args) -> Result<(), Box<dyn StdError + Send + Sync>> {
    if args.migrate_config {
        match config::migrate_json_to_toml() {
            Ok(_) => {
                println!("Successfully migrated configuration from JSON to TOML format.");
                println!("The original JSON config file has been preserved.");
                return Ok(());
            }
            Err(e) => {
                eprintln!("Failed to migrate config: {}", e);
                return Err(e);
            }
        }
    }

    let mut cfg = if let Some(config_path) = &args.config {
        load_or_create_config_from_path(config_path)?
    } else {
        load_or_create_config()?
    };

    if let Some(authors_str) = args.authors {
        cfg.authors = authors_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
    }

    if let Some(theme_color) = args.theme_color {
        cfg.theme_color = theme_color;
    }
    if let Some(max_tries) = args.max_tries {
        cfg.max_tries = max_tries;
    }
    if let Some(log_file) = args.log_file {
        cfg.log_file = log_file;
    }
    if let Some(shell) = args.completion {
        let mut cmd = Args::command();
        generate(shell, &mut cmd, "getquotes", &mut io::stdout());
        return Ok(());
    }
    cfg.rainbow_mode = args.rainbow_mode;

    // Initialize logger
    logger::initialize_logger(&cfg.log_file)?;
    info!("Logger initialized. Log file: {}", cfg.log_file);

    debug!("Loaded config: {:?}", cfg);

    const GIT_HASH: std::option::Option<&str> = try_revision_string!();

    if args.version {
        println!(
            "getquotes v{} \n Commit {}",
            env!("CARGO_PKG_VERSION"),
            GIT_HASH.unwrap_or("Hash Not Found")
        );
        return Ok(());
    }

    let mut rng = thread_rng();

    let color = if cfg.rainbow_mode {
        // Generate random RGB values
        let r: u8 = rng.random();
        let g: u8 = rng.random();
        let b: u8 = rng.random();
        (r, g, b)
    } else {
        // Use parsed theme color
        match parse_hex_color(&cfg.theme_color) {
            Some((r, g, b)) => (r, g, b),
            None => {
                warn!("Invalid hex code in config. Using fallback color.");
                (0x1E, 0x90, 0xFF)
            }
        }
    };

    if args.offline {
        info!("Running in offline mode");
        return display_offline_quote(&cfg, color);
    }

    // Create an HTTP client
    let client = Client::new();
    info!("HTTP client initialized.");

    // Attempt up to max_tries to find a quote
    let max_tries = cfg.max_tries;
    let mut rng = thread_rng();

    for attempt in 1..=max_tries {
        debug!("Attempt {}/{}", attempt, max_tries);
        // Pick a random author from config
        let author_idx = rng.random_range(0..cfg.authors.len());
        let author = &cfg.authors[author_idx];

        info!("Attempting to fetch quote for author: {}", author);

        // Get the page sections for the chosen author
        match quotes::get_author_sections(&client, author).await {
            Ok(Some((title, sections))) => {
                if !sections.is_empty() {
                    let mut found_quote = None;
                    for section in sections {
                        debug!("Fetching quotes from section: {}", section.line);
                        let quotes =
                            match quotes::fetch_quotes(&client, &title, &section.index).await {
                                Ok(q) => q,
                                Err(err) => {
                                    error!("Failed to fetch quotes for author {}: {}", author, err);
                                    continue;
                                }
                            };

                        if !quotes.is_empty() {
                            let random_quote = &quotes[rng.random_range(0..quotes.len())];
                            found_quote = Some((author.to_string(), random_quote.clone()));
                            break;
                        }
                    }
                    if let Some((auth_found, quote_found)) = found_quote {
                        // Apply the parsed hex color to the quote
                        let colorized_quote =
                            format!("\"{}\"", quote_found).truecolor(color.0, color.1, color.2);
                        let dash = "-";

                        // Print the quote and the author on a new line, right-aligned
                        println!(
                            "{}\n\n {:>99}{}",
                            colorized_quote.bold(),
                            dash.bold().green(),
                            auth_found.green()
                        );
                        info!("Quote successfully displayed from author: {}", auth_found);
                        return Ok(());
                    }
                }
            }
            Ok(None) => warn!("No valid page found for author '{}', trying again.", author),
            Err(err) => error!("Failed to get sections for author '{}': {}", author, err),
        }
    }

    error!(
        "Could not find a suitable quote after {} attempts.",
        max_tries
    );
    Err("Failed to retrieve a quote.".into())
}

fn display_offline_quote(
    cfg: &Config,
    color: (u8, u8, u8),
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let cached_quotes = cache::get_cached_quotes()?;

    if cached_quotes.is_empty() {
        error!("No cached quotes available for offline mode");
        return Err(
            "No cached quotes available for offline mode. Please run with --init-cache first."
                .into(),
        );
    }

    let mut rng = thread_rng();

    // Filter quotes by configured authors if specified
    let filtered_quotes: Vec<_> = if !cfg.authors.is_empty() {
        cached_quotes
            .into_iter()
            .filter(|(author, _)| cfg.authors.contains(author))
            .collect()
    } else {
        cached_quotes
    };

    if filtered_quotes.is_empty() {
        error!("No cached quotes found for configured authors");
        return Err("No cached quotes available from configured authors. Try running --init-cache again or check your author list.".into());
    }

    // Select a random quote from filtered list
    let quote_idx = rng.random_range(0..filtered_quotes.len());
    let (author, quote) = &filtered_quotes[quote_idx];

    // Display the randomly selected quote
    let colorized_quote = format!("\"{}\"", quote).truecolor(color.0, color.1, color.2);
    let dash = "-";

    println!(
        "{}\n\n {:>99}{}",
        colorized_quote.bold(),
        dash.bold().green(),
        author.green()
    );
    info!(
        "Offline quote successfully displayed from author: {}",
        author
    );
    Ok(())
}
