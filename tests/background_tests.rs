mod common;

use getquotes::background::update_cache;
use getquotes::cache::{get_cached_quotes, get_database_path, init_cache};
use getquotes::config::{
    Config, default_api_calls_per_minute, default_log_file, default_max_tries, default_prefer_cache,
    default_rainbow_mode, default_theme_color, get_config_path,
};
use mockito::Server;
use reqwest::Client;
use std::sync::Arc;
use tokio::runtime::Runtime;

#[test]
fn test_update_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_home_guard, _temp_dir) = common::setup_temp_home()?;

    // Write a config with the test author so update_cache picks it up
    let config_path = get_config_path()?;
    let config = Config {
        authors: vec!["Test Author".to_string()],
        theme_color: default_theme_color(),
        max_tries: default_max_tries(),
        log_file: default_log_file(),
        rainbow_mode: default_rainbow_mode(),
        prefer_cache: default_prefer_cache(),
        api_calls_per_minute: default_api_calls_per_minute(),
    };
    let toml_string = toml::to_string_pretty(&config)?;
    std::fs::write(&config_path, toml_string)?;

    init_cache()?;

    let mut server = Server::new();

    let _api_guard = common::setup_api_url(&server.url());

    let author_sections_mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "sections".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Test Author".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "parse": {
                "title": "Test Author",
                "sections": [
                    {
                        "index": "1",
                        "number": "1",
                        "line": "Quotes"
                    }
                ]
            }
        }
        "#,
        )
        .expect(1)
        .create();

    let quotes_mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "text".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Test Author".into()),
            mockito::Matcher::UrlEncoded("section".into(), "1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "parse": {
                "title": "Test Author",
                "text": {
                    "*": "<div><ul><li>This is a test quote</li><li>Another test quote</li></ul></div>"
                }
            }
        }
        "#,
        )
        .expect(1)
        .create();

    let rt = Runtime::new()?;
    let client = Arc::new(Client::builder().build()?);

    rt.block_on(update_cache(client))?;

    author_sections_mock.assert();
    quotes_mock.assert();

    let quotes = get_cached_quotes()?;
    assert!(!quotes.is_empty());

    let test_quotes: Vec<_> = quotes
        .iter()
        .filter(|(author, _)| author == "Test Author")
        .collect();
    assert!(!test_quotes.is_empty());

    let quote_texts: Vec<_> = test_quotes.iter().map(|(_, quote)| quote).collect();
    assert!(quote_texts.contains(&&"This is a test quote".to_string()));
    assert!(quote_texts.contains(&&"Another test quote".to_string()));

    // Verify the database path is under the temp home
    let db_path = get_database_path()?;
    assert!(db_path.exists());

    Ok(())
}

