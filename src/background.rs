use crate::cache::get_database_path;
use crate::config::load_or_create_config;
use crate::quotes::{fetch_quotes, get_author_sections};
use reqwest::Client;
use rusqlite::Connection;
use std::error::Error as StdError;
use std::sync::Arc;
use tokio::time;

pub async fn update_cache(
    client: Arc<Client>,
    authors: &[String],
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    if authors.is_empty() {
        return Err("No authors configured for caching.".into());
    }

    for author in authors {
        match get_author_sections(&client, author).await {
            Ok(Some((title, sections))) => {
                for section in sections {
                    match fetch_quotes(&client, &title, &section.index).await {
                        Ok(quotes) => {
                            let db_path = get_database_path()?;
                            let conn = Connection::open(db_path.to_str().unwrap())?;
                            for quote in quotes {
                                match conn.execute(
                                    "INSERT OR IGNORE INTO quotes (author, quote) VALUES (?1, ?2)",
                                    [author.as_str(), &quote],
                                ) {
                                    Ok(_) => println!("Cached quote: {quote}"),
                                    Err(e) => eprintln!("Failed to cache quote: {e}"),
                                }
                            }
                        }
                        Err(e) => eprintln!(
                            "Failed to fetch quotes for section {}: {}",
                            section.index, e
                        ),
                    }
                }
            }
            Ok(None) => println!("No valid page found for author '{author}'."),
            Err(e) => eprintln!("Failed to get sections for author '{author}': {e}"),
        }
    }
    Ok(())
}

pub async fn cache_quotes(client: Arc<Client>) {
    loop {
        match load_or_create_config() {
            Ok(cfg) => {
                if let Err(e) = update_cache(client.clone(), &cfg.authors).await {
                    eprintln!("Error updating cache: {e}");
                }
            }
            Err(e) => eprintln!("Error loading config: {e}"),
        }
        time::sleep(time::Duration::from_secs(24 * 3600)).await;
    }
}
