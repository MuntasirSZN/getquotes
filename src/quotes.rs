use crate::types::{QueryResult, Section};
use log::trace;
use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error as StdError;

// Get API base URL from environment or use default
fn get_api_base_url() -> String {
    std::env::var("WIKIQUOTE_API_URL").unwrap_or_else(|_| "https://en.wikiquote.org".to_string())
}

pub async fn get_author_sections(
    client: &Client,
    author: &str,
) -> Result<Option<(String, Vec<Section>)>, Box<dyn StdError + Send + Sync>> {
    let author_encoded = urlencoding::encode(author);
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=sections&page={}",
        get_api_base_url(),
        author_encoded
    );

    trace!("Fetching author sections from URL: {}", api_url);
    let res = client.get(&api_url).send().await?;
    if res.status().is_success() {
        let val: serde_json::Value = res.json().await?;
        if val.get("parse").is_none() {
            return Ok(None);
        }
        let query: QueryResult = serde_json::from_value(val)?;
        let page_title = query.parse.title;
        let some_sections = query.parse.sections.unwrap_or_default();
        return Ok(Some((page_title, some_sections)));
    }
    Ok(None)
}

pub async fn fetch_quotes(
    client: &Client,
    title: &str,
    section: &str,
) -> Result<Vec<String>, Box<dyn StdError + Send + Sync>> {
    let title_encoded = urlencoding::encode(title);
    let api_url = format!(
        "{}/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
        get_api_base_url(),
        title_encoded,
        section
    );

    trace!("Fetching quotes from URL: {}", api_url);
    let res = client.get(&api_url).send().await?;
    if !res.status().is_success() {
        return Err(format!("Failed to fetch quotes: {}", res.status()).into());
    }

    let val: serde_json::Value = res.json().await?;
    if val.get("parse").is_none() {
        return Ok(vec![]);
    }
    let html_content = val["parse"]["text"]["*"].as_str().unwrap_or("");
    let document = Html::parse_document(html_content);

    let selector = Selector::parse("li:not(li li)").unwrap();
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

    Ok(quotes)
}
