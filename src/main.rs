mod background;
mod cache;
mod config;
mod quotes;
mod types;

use clap::Parser;
use getquotes::cli::Args;
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
        let client_clone = client.clone();
        tokio::spawn(async move {
            background::cache_quotes(client_clone).await;
        });
    }

    run(args).await
}
