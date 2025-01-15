use getquotes::quotes::{fetch_quotes, get_author_sections};
use mockito::Server;
use reqwest::Client;

mod common;

#[tokio::test]
async fn test_author_sections_fetching() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/w/api.php")
        .with_status(200)
        .with_body(
            r#"{
            "parse": {
                "title": "Einstein",
                "sections": [{"index": "1", "line": "Quotes"}]
            }
        }"#,
        )
        .create();

    let client = Client::new();
    let result = get_author_sections(&client, "Einstein").await?;
    assert!(result.is_some());
    Ok(())
}

#[tokio::test]
async fn test_quotes_fetching() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut server = Server::new();
    let _m = server
        .mock("GET", "/w/api.php")
        .with_status(200)
        .with_body(
            r#"{
            "parse": {
                "text": {
                    "*": "<ul><li>Test quote 1</li><li>Test quote 2</li></ul>"
                }
            }
        }"#,
        )
        .create();

    let client = Client::new();
    let quotes = fetch_quotes(&client, "Einstein", "1").await?;
    assert_eq!(quotes.len(), 2);
    Ok(())
}
