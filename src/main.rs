mod background;
mod cache;
mod config;
mod quotes;
mod types;

use clap::Parser;
use getquotes::{run, Args};
use reqwest::Client;
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::spawn;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let args = Args::parse();
    let client = Arc::new(Client::new());

    if args.init_cache {
        cache::init_cache()?;
        // Start background cache population with shared client
        let client_clone = client.clone();
        spawn(async move {
            background::cache_quotes(client_clone).await;
        });
    }

    // Pass other arguments to the run function
    run(args).await
}
