use clap::Parser;
use getquotes::cli::Args;
use getquotes::{background, cache};
use getquotes::run;
use reqwest::Client;
use std::error::Error as StdError;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let args = Args::parse();
    let client = Arc::new(Client::new());

    if args.init_cache {
        cache::init_cache()?;
        if let Err(e) = background::update_cache(client.clone()).await {
            eprintln!("Warning: Failed to populate cache: {e}");
        }
        return Ok(());
    }

    run(args).await
}
