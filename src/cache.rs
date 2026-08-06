use rusqlite::{Connection, Result};
use simd_normalizer::UnicodeNormalization;
use std::collections::HashMap;
use std::env::home_dir;
use std::error::Error as StdError;
use std::fs::create_dir_all;
use std::path::PathBuf;

/// Normalize a quote with NFKC so typographic variants (ligatures, fullwidth
/// forms, composed/decomposed accents) dedupe as the same quote text.
pub fn normalize_quote(quote: &str) -> String {
    quote.nfkc().into_owned()
}

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

    // Migrate existing rows: normalize quotes (NFKC) and collapse rows that
    // become identical after normalization, keeping the lowest id.
    let mut stmt = conn.prepare("SELECT id, quote FROM quotes")?;
    let rows: Vec<(i64, String)> = stmt
        .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
        .collect::<rusqlite::Result<_>>()?;

    let mut keep: HashMap<String, i64> = HashMap::new();
    for (id, quote) in &rows {
        keep.entry(normalize_quote(quote)).or_insert(*id);
    }

    // Phase 1: delete rows that duplicate a lower-id row after normalization.
    for (id, quote) in &rows {
        if keep[&normalize_quote(quote)] != *id {
            conn.execute("DELETE FROM quotes WHERE id = ?1", [id])?;
        }
    }

    // Phase 2: store the normalized form. No clash can occur: a clash would
    // mean two surviving rows in the same normalized group, but each group
    // keeps exactly one (the lowest id).
    let survivors: Vec<(i64, String)> = {
        let mut stmt = conn.prepare("SELECT id, quote FROM quotes")?;
        stmt.query_map([], |row| Ok((row.get(0)?, row.get(1)?)))?
            .collect::<rusqlite::Result<_>>()?
    };
    for (id, quote) in survivors {
        let normalized = normalize_quote(&quote);
        if normalized != quote {
            conn.execute(
                "UPDATE quotes SET quote = ?1 WHERE id = ?2",
                rusqlite::params![&normalized, id],
            )?;
        }
    }

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
    let selected_quote = fastrand::choice(&filtered_quotes).cloned();

    Ok(selected_quote)
}
