pub mod background;
pub mod cache;
pub mod cli;
pub mod config;
pub mod logger;
pub mod quotes;
pub mod render;
pub mod throttle;
pub mod types;

use crate::cli::Args;
use crate::config::Config;
use crate::config::{load_or_create_config, load_or_create_config_from_path};
use crate::throttle::ApiThrottler;
use clap::CommandFactory;
use clap_complete::generate;
use git_rev::try_revision_string;
use log::{debug, error, info, warn};
use rand::{RngExt, rng as thread_rng};
use reqwest::Client;
use std::error::Error as StdError;
use std::io;

pub async fn run(args: Args) -> Result<(), Box<dyn StdError + Send + Sync>> {
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
    if args.rainbow_mode.unwrap_or(false) {
        cfg.rainbow_mode = true;
    }

    // Initialize logger
    logger::initialize_logger(&cfg.log_file)?;
    info!("Logger initialized. Log file: {}", cfg.log_file);

    debug!("Loaded config: {cfg:?}");

    const GIT_HASH: std::option::Option<&str> = try_revision_string!();

    if args.version {
        println!(
            "getquotes v{} \n Commit {}",
            env!("CARGO_PKG_VERSION"),
            GIT_HASH.unwrap_or("Hash Not Found")
        );
        return Ok(());
    }

    if args.offline {
        info!("Running in offline mode");
        return display_offline_quote(&cfg);
    }

    // Try cache first if prefer_cache is enabled (randomized data approach)
    if cfg.prefer_cache {
        info!("Trying to get quote from cache first (randomized data mode)");
        match cache::get_random_cached_quote(&cfg.authors) {
            Ok(Some((author, quote))) => {
                // Successfully got a quote from cache
                println!("{}", render::render_output(&cfg, &quote, &author));
                info!("Quote successfully displayed from cache: {author}");
                return Ok(());
            }
            Ok(None) => {
                info!("No suitable quotes found in cache, falling back to API");
            }
            Err(err) => {
                warn!("Failed to access cache: {err}, falling back to API");
            }
        }
    }

    // Create an HTTP client and throttler for API calls
    let client = Client::new();
    let mut throttler = ApiThrottler::new(cfg.api_calls_per_minute);
    info!(
        "HTTP client initialized. API rate limit: {} calls/minute",
        cfg.api_calls_per_minute
    );

    // Attempt up to max_tries to find a quote
    let max_tries = cfg.max_tries;
    let mut rng = thread_rng();

    for attempt in 1..=max_tries {
        debug!("Attempt {attempt}/{max_tries}");
        // Pick a random author from config
        let author_idx = rng.random_range(0..cfg.authors.len());
        let author = &cfg.authors[author_idx];

        info!("Attempting to fetch quote for author: {author}");

        // Apply throttling before making API call
        throttler.throttle().await;

        // Get the page sections for the chosen author
        match quotes::get_author_sections(&client, author).await {
            Ok(Some((title, sections))) => {
                if !sections.is_empty() {
                    let mut found_quote = None;
                    for section in sections {
                        debug!("Fetching quotes from section: {}", section.line);

                        // Apply throttling before making API call
                        throttler.throttle().await;

                        let quotes =
                            match quotes::fetch_quotes(&client, &title, &section.index).await {
                                Ok(q) => q,
                                Err(err) => {
                                    error!("Failed to fetch quotes for author {author}: {err}");
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
                        println!("{}", render::render_output(&cfg, &quote_found, &auth_found));
                        info!("Quote successfully displayed from author: {auth_found}");
                        return Ok(());
                    }
                }
            }
            Ok(None) => warn!("No valid page found for author '{author}', trying again."),
            Err(err) => error!("Failed to get sections for author '{author}': {err}"),
        }
    }

    error!("Could not find a suitable quote after {max_tries} attempts.");
    Err("Failed to retrieve a quote.".into())
}

fn display_offline_quote(cfg: &Config) -> Result<(), Box<dyn StdError + Send + Sync>> {
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
    println!("{}", render::render_output(cfg, quote, author));
    info!("Offline quote successfully displayed from author: {author}");
    Ok(())
}
