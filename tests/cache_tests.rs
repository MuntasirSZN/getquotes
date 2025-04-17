mod common;

use getquotes::cache::{get_cached_quotes, get_database_path, init_cache};
use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;

#[test]
fn test_get_database_path() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let db_path = get_database_path()?;

    let expected_suffix = PathBuf::from(".local/share/getquotes/quotes.db");
    assert!(db_path.ends_with(expected_suffix));

    Ok(())
}

#[test]
fn test_init_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    init_cache()?;

    let db_path = get_database_path()?;

    assert!(db_path.exists());

    let conn = Connection::open(db_path)?;
    let mut stmt =
        conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='quotes'")?;
    let table_exists = stmt.query_row([], |_| Ok(true)).is_ok();

    assert!(table_exists);

    let mut pragma_stmt = conn.prepare("PRAGMA table_info(quotes)")?;
    let columns: SqliteResult<Vec<(i32, String, String, i32, Option<String>, i32)>> = pragma_stmt
        .query_map([], |row| {
            Ok((
                row.get(0)?, // cid
                row.get(1)?, // name
                row.get(2)?, // type
                row.get(3)?, // notnull
                row.get(4)?, // dflt_value
                row.get(5)?, // pk
            ))
        })?
        .collect();

    let columns = columns?;
    assert_eq!(columns.len(), 3); // id, author, quote

    // id column should be INTEGER and PRIMARY KEY
    assert_eq!(columns[0].1, "id");
    assert_eq!(columns[0].2, "INTEGER");
    assert_eq!(columns[0].5, 1); // pk flag

    // author column should be TEXT
    assert_eq!(columns[1].1, "author");
    assert_eq!(columns[1].2, "TEXT");

    // quote column should be TEXT and UNIQUE
    assert_eq!(columns[2].1, "quote");
    assert_eq!(columns[2].2, "TEXT");

    Ok(())
}

#[test]
fn test_get_cached_quotes() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    init_cache()?;

    let db_path = get_database_path()?;
    let conn = Connection::open(&db_path)?;

    // Insert some test quotes
    conn.execute(
        "INSERT INTO quotes (author, quote) VALUES (?1, ?2)",
        [
            "Albert Einstein",
            "Imagination is more important than knowledge.",
        ],
    )?;

    conn.execute(
        "INSERT INTO quotes (author, quote) VALUES (?1, ?2)",
        [
            "Mark Twain",
            "The secret of getting ahead is getting started.",
        ],
    )?;

    let quotes = get_cached_quotes()?;

    // Verify quotes were retrieved correctly
    assert_eq!(quotes.len(), 2);

    // Find Einstein quote
    let einstein_quote = quotes
        .iter()
        .find(|(author, _)| author == "Albert Einstein");
    assert!(einstein_quote.is_some());
    assert_eq!(
        einstein_quote.unwrap().1,
        "Imagination is more important than knowledge."
    );

    // Find Twain quote
    let twain_quote = quotes.iter().find(|(author, _)| author == "Mark Twain");
    assert!(twain_quote.is_some());
    assert_eq!(
        twain_quote.unwrap().1,
        "The secret of getting ahead is getting started."
    );

    // Test with no quotes
    let conn = Connection::open(db_path)?;
    conn.execute("DELETE FROM quotes", [])?;

    let empty_quotes = get_cached_quotes()?;
    assert_eq!(empty_quotes.len(), 0);

    Ok(())
}
