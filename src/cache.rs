use rand::prelude::*;
use rusqlite::{Connection, Result};
use std::env::home_dir;
use std::error::Error as StdError;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn get_database_path() -> Result<PathBuf, Box<dyn StdError + Send + Sync>> {
    let home = home_dir().ok_or("Unable to find home directory.")?;
    let db_path = home.join(".local/share/getquotes/quotes.db");
    if let Some(parent_dir) = db_path.parent() {
        create_dir_all(parent_dir)?;
    }
    Ok(db_path)
}

pub fn init_cache() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let db_path = get_database_path()?;
    let conn = Connection::open(db_path.to_str().unwrap())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quotes (id INTEGER PRIMARY KEY, author TEXT, quote TEXT UNIQUE)",
        [],
    )?;
    Ok(())
}

pub fn get_cached_quotes() -> Result<Vec<(String, String)>, Box<dyn StdError + Send + Sync>> {
    let db_path = get_database_path()?;
    let conn = Connection::open(db_path.to_str().unwrap())?;
    let mut stmt = conn.prepare("SELECT author, quote FROM quotes")?;
    let quote_iter = stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?;

    let mut quotes = Vec::new();
    for quote in quote_iter {
        quotes.push(quote?);
    }
    Ok(quotes)
}

pub fn get_random_cached_quote(
    authors: &[String],
) -> Result<Option<(String, String)>, Box<dyn StdError + Send + Sync>> {
    let cached_quotes = get_cached_quotes()?;

    if cached_quotes.is_empty() {
        return Ok(None);
    }

    // Filter quotes by specified authors if provided
    let filtered_quotes: Vec<_> = if !authors.is_empty() {
        cached_quotes
            .into_iter()
            .filter(|(author, _)| authors.contains(author))
            .collect()
    } else {
        cached_quotes
    };

    if filtered_quotes.is_empty() {
        return Ok(None);
    }

    // Get a random quote
    let mut rng = rand::rng();
    let selected_quote = filtered_quotes.choose(&mut rng).cloned();

    Ok(selected_quote)
}
