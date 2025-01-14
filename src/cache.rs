use rusqlite::{Connection, Result};
use std::error::Error as StdError;

pub fn init_cache() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let conn = Connection::open("quotes.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quotes (id INTEGER PRIMARY KEY, author TEXT, quote TEXT)",
        [],
    )?;
    Ok(())
}

pub fn get_cached_quotes() -> Result<Vec<String>, Box<dyn StdError + Send + Sync>> {
    let conn = Connection::open("quotes.db")?;
    let mut stmt = conn.prepare("SELECT quote FROM quotes")?;
    let quote_iter = stmt.query_map([], |row| row.get(0))?;

    let mut quotes = Vec::new();
    for quote in quote_iter {
        quotes.push(quote?);
    }
    Ok(quotes)
}
