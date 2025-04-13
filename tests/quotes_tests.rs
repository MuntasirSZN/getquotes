mod common;

use mockito::Server;
use reqwest::blocking::Client;

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

    // Set environment variable to use our mock server
    unsafe { std::env::set_var("WIKIQUOTE_API_URL", server.url()) };

    let client = Client::new();
    let author_encoded = urlencoding::encode("Albert Einstein");
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=sections&page={}",
        server.url(),
        author_encoded
    );

    let res = client.get(&api_url).send().unwrap();
    assert!(res.status().is_success());

    let val: serde_json::Value = res.json().unwrap();
    assert!(val.get("parse").is_some());

    let query: getquotes::types::QueryResult = serde_json::from_value(val).unwrap();
    assert_eq!(query.parse.title, "Albert Einstein");

    let some_sections = query.parse.sections.unwrap_or_default();
    assert_eq!(some_sections.len(), 2);
    assert_eq!(some_sections[0].line, "Quotes");
    assert_eq!(some_sections[1].line, "Misattributed");

    // Validate mock assertions before ending test
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

    let client = Client::new();
    let author_encoded = urlencoding::encode("NonExistentAuthor");
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=sections&page={}",
        server.url(),
        author_encoded
    );

    let res = client.get(&api_url).send().unwrap();
    assert!(res.status().is_success());

    let val: serde_json::Value = res.json().unwrap();
    assert!(val.get("parse").is_none());
    assert!(val.get("error").is_some());

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

    // Set environment variable to use our mock server
    unsafe { std::env::set_var("WIKIQUOTE_API_URL", server.url()) };

    let client = Client::new();
    let title_encoded = urlencoding::encode("Albert Einstein");
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
        server.url(),
        title_encoded,
        "1"
    );

    let res = client.get(&api_url).send().unwrap();
    assert!(res.status().is_success());

    let val: serde_json::Value = res.json().unwrap();
    assert!(val.get("parse").is_some());

    let html_content = val["parse"]["text"]["*"].as_str().unwrap_or("");
    let document = scraper::Html::parse_document(html_content);

    let selector = scraper::Selector::parse("li:not(li li)").unwrap();
    let mut quotes = Vec::new();

    for element in document.select(&selector) {
        let text_content = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text_content.is_empty() {
            quotes.push(text_content);
        }
    }

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

    // Set environment variable to use our mock server
    unsafe { std::env::set_var("WIKIQUOTE_API_URL", server.url()) };

    let client = Client::new();
    let title_encoded = urlencoding::encode("Albert Einstein");
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
        server.url(),
        title_encoded,
        "3"
    );

    let res = client.get(&api_url).send().unwrap();
    assert!(res.status().is_success());

    let val: serde_json::Value = res.json().unwrap();
    assert!(val.get("parse").is_some());

    let html_content = val["parse"]["text"]["*"].as_str().unwrap_or("");
    let document = scraper::Html::parse_document(html_content);

    let selector = scraper::Selector::parse("li:not(li li)").unwrap();
    let mut quotes = Vec::new();

    for element in document.select(&selector) {
        let text_content = element
            .text()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        if !text_content.is_empty() {
            quotes.push(text_content);
        }
    }

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

    // Make sure to set the environment variable for the API URL
    unsafe { std::env::set_var("WIKIQUOTE_API_URL", server.url()) };

    let client = Client::new();
    let title_encoded = urlencoding::encode("Albert Einstein");
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
        server.url(),
        title_encoded,
        "4"
    );

    let res = client.get(&api_url).send().unwrap();
    assert_eq!(res.status().as_u16(), 501);

    // Verify the mock was called
    mock.assert();
}
