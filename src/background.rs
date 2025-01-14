use crate::config::load_or_create_config;
use crate::quotes::{fetch_quotes, get_author_sections};
use rand::Rng;
use reqwest::Client;
use rusqlite::Connection;
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::time;

pub async fn update_cache(client: Arc<Client>) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let cfg = load_or_create_config()?;
    let author_idx = rand::thread_rng().gen_range(0..cfg.authors.len());
    let author = &cfg.authors[author_idx];

    if let Ok(Some((title, sections))) = get_author_sections(&client, author).await {
        for section in sections {
            let quotes = fetch_quotes(&client, &title, &section.index).await?;
            for quote in quotes {
                // Store the quote in the database
                let conn = Connection::open("quotes.db")?;
                conn.execute(
                    "INSERT INTO quotes (author, quote) VALUES (?1, ?2)",
                    &[&author, &quote],
                )?;
            }
        }
    }
    Ok(())
}

pub async fn cache_quotes(client: Arc<Client>) {
    loop {
        if let Err(e) = update_cache(client.clone()).await {
            eprintln!("Error updating cache: {}", e);
        }
        time::sleep(time::Duration::from_secs(24 * 3600)).await;
    }
}
