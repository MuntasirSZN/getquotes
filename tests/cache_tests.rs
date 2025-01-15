mod common;

use getquotes::cache::get_cached_quotes;
use rusqlite::Connection;

#[test]
pub fn test_cache_operations() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let conn = Connection::open(":memory:")?;
    conn.execute(
        "CREATE TABLE quotes (id INTEGER PRIMARY KEY, author TEXT, quote TEXT)",
        [],
    )?;

    conn.execute(
        "INSERT INTO quotes (author, quote) VALUES (?1, ?2)",
        ["Test Author", "Test Quote"],
    )?;

    let quotes = get_cached_quotes()?;
    assert!(!quotes.is_empty());
    Ok(())
}
