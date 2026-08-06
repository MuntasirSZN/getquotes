use crate::cache::get_database_path;
use crate::config::load_or_create_config;
use crate::quotes::{fetch_quotes, get_author_sections};
use reqwest::Client;
use rusqlite::Connection;
use std::collections::HashSet;
use std::error::Error as StdError;
use std::io::Write;
use std::sync::Arc;
use tokio::time;

pub async fn update_cache(
    client: Arc<Client>,
    authors: &[String],
) -> Result<(), Box<dyn StdError + Send + Sync>> {
    if authors.is_empty() {
        return Err("No authors configured for caching.".into());
    }

    let theme_color = load_or_create_config()
        .map(|cfg| cfg.theme_color)
        .unwrap_or_default();
    let paint = |text: &str| crate::render::paint_theme(text, &theme_color);

    let mut total_cached = 0;
    let mut total_duplicates = 0;
    let mut total_failed = 0;

    for author in authors {
        println!("{}", paint(&format!("Fetching quotes for '{author}'...")));
        match get_author_sections(&client, author).await {
            Ok(Some((title, sections))) => {
                let db_path = get_database_path()?;
                let conn = Connection::open(db_path.to_str().unwrap())?;

                // Dedupe quotes within this run (same quote appears across sections).
                let mut seen: HashSet<String> = HashSet::new();
                let mut author_cached = 0;
                let mut author_duplicates = 0;
                let mut author_failed = 0;

                for (i, section) in sections.iter().enumerate() {
                    // Clear the line first so shorter progress overwrites cleanly.
                    print!(
                        "\x1b[2K\r{}",
                        paint(&format!(
                            "Fetching section {}/{} for '{author}'...",
                            i + 1,
                            sections.len()
                        ))
                    );
                    std::io::stdout().flush()?;

                    match fetch_quotes(&client, &title, &section.index).await {
                        Ok(quotes) => {
                            for quote in quotes {
                                let normalized = crate::cache::normalize_quote(&quote);
                                if !seen.insert(normalized.clone()) {
                                    continue; // already fetched for this author this run
                                }
                                match conn.execute(
                                    "INSERT OR IGNORE INTO quotes (author, quote) VALUES (?1, ?2)",
                                    [author.as_str(), &normalized],
                                ) {
                                    Ok(rows_affected) if rows_affected > 0 => author_cached += 1,
                                    Ok(_) => author_duplicates += 1,
                                    Err(_) => author_failed += 1,
                                }
                                print!(
                                    "\x1b[2K\r{}",
                                    crate::render::paint_progress_line(
                                        author_cached,
                                        author,
                                        &theme_color,
                                        "for"
                                    )
                                );
                                std::io::stdout().flush()?;
                            }
                        }
                        Err(e) => eprintln!(
                            "\nFailed to fetch quotes for section {}: {}",
                            section.index, e
                        ),
                    }
                }
                print!("\x1b[2K\r");
                let mut summary =
                    crate::render::paint_progress_line(author_cached, author, &theme_color, "from");
                if author_duplicates > 0 {
                    summary.push_str(&paint(&format!(" ({author_duplicates} already cached)")));
                }
                if author_failed > 0 {
                    summary.push_str(&paint(&format!(", {author_failed} failed")));
                }
                println!("{}", paint(&summary));
                total_cached += author_cached;
                total_duplicates += author_duplicates;
                total_failed += author_failed;
            }
            Ok(None) => println!(
                "{}",
                paint(&format!("No valid page found for author '{author}'."))
            ),
            Err(e) => eprintln!("Failed to get sections for author '{author}': {e}"),
        }
    }

    println!(
        "{}",
        paint(&format!(
            "Done: {total_cached} quotes cached, {total_duplicates} duplicates, {total_failed} failures"
        ))
    );
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
