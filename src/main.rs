use clap::Parser;
use getquotes::cli::Args;
use getquotes::run;
use getquotes::{background, cache, config};
use reqwest::Client;
use std::error::Error as StdError;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let args = Args::parse();
    let client = Arc::new(Client::new());

    if let Some(init_cache_val) = &args.init_cache {
        cache::init_cache()?;
        let authors: Vec<String> = if init_cache_val.is_empty() {
            config::load_or_create_config()?.authors
        } else {
            init_cache_val
                .split(',')
                .map(|s| s.trim().to_string())
                .collect()
        };
        if let Err(e) = background::update_cache(client.clone(), &authors).await {
            eprintln!("Warning: Failed to populate cache: {e}");
        }
        return Ok(());
    }

    run(args).await
}
