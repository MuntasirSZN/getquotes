mod common;

use getquotes::quotes::{fetch_quotes, get_author_sections};
use mockito::Server;
use reqwest::Client;
use tokio::runtime::Runtime;

#[test]
fn test_get_author_sections_success() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "sections".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Albert Einstein".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "parse": {
                "title": "Albert Einstein",
                "sections": [
                    {
                        "index": "1",
                        "number": "1",
                        "line": "Quotes"
                    },
                    {
                        "index": "2",
                        "number": "2",
                        "line": "Misattributed"
                    }
                ]
            }
        }
        "#,
        )
        .expect(1)
        .create();

    let _api_guard = common::setup_api_url(&server.url());

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let result = rt
        .block_on(get_author_sections(&client, "Albert Einstein"))
        .unwrap();

    assert!(result.is_some());
    let (title, sections) = result.unwrap();
    assert_eq!(title, "Albert Einstein");
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].line, "Quotes");
    assert_eq!(sections[1].line, "Misattributed");

    mock.assert();
}

#[test]
fn test_get_author_sections_not_found() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "sections".into()),
            mockito::Matcher::UrlEncoded("page".into(), "NonExistentAuthor".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "error": {
                "code": "missingtitle",
                "info": "The page you specified doesn't exist."
            }
        }
        "#,
        )
        .create();

    let _api_guard = common::setup_api_url(&server.url());

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let result = rt
        .block_on(get_author_sections(&client, "NonExistentAuthor"))
        .unwrap();

    assert!(result.is_none());

    mock.assert();
}

#[test]
fn test_fetch_quotes_success() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "text".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Albert Einstein".into()),
            mockito::Matcher::UrlEncoded("section".into(), "1".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "parse": {
                "title": "Albert Einstein",
                "text": {
                    "*": "<div><ul><li>Quote 1</li><li>Quote 2</li></ul></div>"
                }
            }
        }
        "#,
        )
        .expect(1)
        .create();

    let _api_guard = common::setup_api_url(&server.url());

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let quotes = rt
        .block_on(fetch_quotes(&client, "Albert Einstein", "1"))
        .unwrap();

    assert_eq!(quotes.len(), 2);
    assert_eq!(quotes[0], "Quote 1");
    assert_eq!(quotes[1], "Quote 2");

    mock.assert();
}

#[test]
fn test_fetch_quotes_empty() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "text".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Albert Einstein".into()),
            mockito::Matcher::UrlEncoded("section".into(), "3".into()),
        ]))
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(
            r#"
        {
            "parse": {
                "title": "Albert Einstein",
                "text": {
                    "*": "<div><ul></ul></div>"
                }
            }
        }
        "#,
        )
        .expect(1)
        .create();

    let _api_guard = common::setup_api_url(&server.url());

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let quotes = rt
        .block_on(fetch_quotes(&client, "Albert Einstein", "3"))
        .unwrap();

    assert_eq!(quotes.len(), 0);

    mock.assert();
}

#[test]
fn test_fetch_quotes_bad_response() {
    let mut server = Server::new();
    let mock = server
        .mock("GET", "/w/api.php")
        .match_query(mockito::Matcher::AllOf(vec![
            mockito::Matcher::UrlEncoded("action".into(), "parse".into()),
            mockito::Matcher::UrlEncoded("format".into(), "json".into()),
            mockito::Matcher::UrlEncoded("prop".into(), "text".into()),
            mockito::Matcher::UrlEncoded("page".into(), "Albert Einstein".into()),
            mockito::Matcher::UrlEncoded("section".into(), "4".into()),
        ]))
        .with_status(501)
        .expect(1)
        .create();

    let _api_guard = common::setup_api_url(&server.url());

    let rt = Runtime::new().unwrap();
    let client = Client::new();
    let result = rt.block_on(fetch_quotes(&client, "Albert Einstein", "4"));

    assert!(result.is_err());

    mock.assert();
}
