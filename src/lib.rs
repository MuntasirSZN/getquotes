mod background;
mod cache;
mod config;
mod logger;
mod quotes;
mod types;

use crate::config::{load_or_create_config, parse_hex_color};
use clap::Parser;
use colored::*;
use log::{debug, error, info, warn};
use rand::{thread_rng, Rng};
use reqwest::Client;
use std::error::Error as StdError;
use std::sync::Arc;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long, help = "Specify a list of authors to fetch quotes from")]
    pub authors: Option<String>,

    #[arg(long, help = "Set the theme color for the displayed quotes")]
    pub theme_color: Option<String>,

    #[arg(long, help = "Set the maximum number of tries to fetch a quote")]
    pub max_tries: Option<usize>,

    #[arg(long, help = "Specify the log file path")]
    log_file: Option<String>,

    #[arg(long, help = "Enable rainbow mode for random quote colors")]
    rainbow_mode: bool,

    #[arg(long, help = "Initialize the quote cache for offline mode")]
    pub init_cache: bool,

    #[arg(long, help = "Run in offline mode, using cached quotes")]
    offline: bool,
}

pub async fn run(args: Args, client: Arc<Client>) -> Result<(), Box<dyn StdError + Send + Sync>> {
    // Load or create config file
    let mut cfg = load_or_create_config()?;

    // Update config with CLI options
    if let Some(authors) = args.authors {
        cfg.authors = authors.split(',').map(|s| s.to_string()).collect();
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
    cfg.rainbow_mode = args.rainbow_mode;

    // Initialize logger
    logger::initialize_logger(&cfg.log_file)?;
    info!("Logger initialized. Log file: {}", cfg.log_file);

    debug!("Loaded config: {:?}", cfg);

    let mut rng = thread_rng();

    let color = if cfg.rainbow_mode {
        // Generate random RGB values
        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
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

    // Create an HTTP client
    let client = Client::new();
    info!("HTTP client initialized.");

    // Attempt up to max_tries to find a quote
    let max_tries = cfg.max_tries;
    let mut rng = thread_rng();

    for attempt in 1..=max_tries {
        debug!("Attempt {}/{}", attempt, max_tries);
        // Pick a random author from config
        let author_idx = rng.gen_range(0..cfg.authors.len());
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
                            let random_quote = &quotes[rng.gen_range(0..quotes.len())];
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
