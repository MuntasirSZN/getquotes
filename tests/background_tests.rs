mod common;

use getquotes::cache::{get_cached_quotes, get_database_path, init_cache};
use getquotes::config::{
    Config, default_log_file, default_max_tries, default_rainbow_mode, default_theme_color,
};
use mockito::Server;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time;

use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref CONFIG_AUTHORS: Mutex<Vec<String>> = Mutex::new(vec!["Test Author".to_string()]);
}

#[test]
fn test_update_cache() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    init_cache()?;

    let mut server = Server::new();

    unsafe { std::env::set_var("WIKIQUOTE_API_URL", server.url()) };

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

    let quotes_mock = server.mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "text".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Test Author".into()),
            mockito::Matcher::UrlEncoded("section".into(), "1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"
        {
            "parse": {
                "title": "Test Author",
                "text": {
                    "*": "<div><ul><li>This is a test quote</li><li>Another test quote</li></ul></div>"
                }
            }
        }
        "#)
        .expect(1)
        .create();

    let rt = Runtime::new()?;

    let client = Arc::new(Client::builder().build()?);

    {
        let mut authors = CONFIG_AUTHORS.lock().unwrap();
        *authors = vec!["Test Author".to_string()];
    }

    rt.block_on(async {
        let mock_config = Config {
            authors: vec!["Test Author".to_string()],
            theme_color: default_theme_color(),
            max_tries: default_max_tries(),
            log_file: default_log_file(),
            rainbow_mode: default_rainbow_mode(),
        };

        let author = &mock_config.authors[0];

        let author_encoded = urlencoding::encode(author);
        let api_url = format!(
            "{}/w/api.php?action=parse&format=json&prop=sections&page={}",
            server.url(), author_encoded
        );

        let res = client.get(&api_url).send().await?;
        if res.status().is_success() {
            let val: serde_json::Value = res.json().await?;
            if val.get("parse").is_some() {
                let query: getquotes::types::QueryResult = serde_json::from_value(val)?;
                let page_title = query.parse.title;
                let some_sections = query.parse.sections.unwrap_or_default();

                for section in &some_sections {
                    let title_encoded = urlencoding::encode(&page_title);
                    let quotes_url = format!(
                        "{}/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
                        server.url(), title_encoded, section.index
                    );

                    let res = client.get(&quotes_url).send().await?;
                    if res.status().is_success() {
                        let val: serde_json::Value = res.json().await?;
                        if val.get("parse").is_some() {
                            let html_content = val["parse"]["text"]["*"].as_str().unwrap_or("");
                            let document = scraper::Html::parse_document(html_content);

                            let selector = scraper::Selector::parse("li:not(li li)").unwrap();

                            for element in document.select(&selector) {
                                let text_content = element
                                    .text()
                                    .collect::<Vec<_>>()
                                    .join(" ")
                                    .trim()
                                    .to_string();

                                if !text_content.is_empty() {
                                    let db_path = get_database_path()?;
                                    let conn = rusqlite::Connection::open(db_path.to_str().unwrap())?;
                                    conn.execute(
                                        "INSERT OR IGNORE INTO quotes (author, quote) VALUES (?1, ?2)",
                                        [author, &text_content],
                                    )?;
                                }
                            }
                        }
                    }
                }
            }
        }

        time::sleep(Duration::from_millis(500)).await;

        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })?;

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

    Ok(())
}
