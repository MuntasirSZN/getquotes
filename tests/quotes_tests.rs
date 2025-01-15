mod common;

use getquotes::quotes::{fetch_quotes, get_author_sections};
use mockito::Server;
use reqwest::Client;
use std::error::Error as StdError;

#[tokio::test]
async fn test_author_sections_fetching() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let mut server = Server::new_async().await;
    let _m = server.mock("GET", "/w/api.php")
        .with_status(200)
        .with_body(r#"{ "parse": { "title": "Einstein", "sections": [{"index": "1", "line": "Quotes"}] } }"#)
        .create_async().await;

    let client = Client::new();
    let result = get_author_sections(&client, "Einstein").await?;
    assert!(result.is_some());
    Ok(())
}

#[tokio::test]
async fn test_quotes_fetching() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let mut server = mockito::Server::new_async().await;
    let mock = server
        .mock("GET", "/w/api.php")
        .with_status(200)
        .with_body(
            r#"{"parse": {"text": {"*": "<ul><li>Test quote 1</li><li>Test quote 2</li></ul>"}}}"#,
        )
        .create_async();

    let client = reqwest::Client::new();
    let quotes = fetch_quotes(&client, "Einstein", "1").await?;
    assert_eq!(quotes.len(), 2);
    Ok(())
}
