use colored::*;
use dirs::home_dir;
use log::{debug, error, info, trace, warn};
use rand::{thread_rng, Rng};
use reqwest::Client;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

/// Structure matching our config file: ~/.config/getquotes/config.json
/// Now includes additional fields like 'max_tries' and 'log_file'.
#[derive(Debug, Serialize, Deserialize)]
struct Config {
    authors: Vec<String>,
    #[serde(default = "default_theme_color")]
    theme_color: String,
    #[serde(default = "default_max_tries")]
    max_tries: usize,
    #[serde(default = "default_log_file")]
    log_file: String,
}

/// Provide a default hex color if the field is missing in the config file.
fn default_theme_color() -> String {
    "#20B2AA".to_string()
}

/// Provide a default maximum number of tries if the field is missing.
fn default_max_tries() -> usize {
    30
}

/// Provide a default log file path if the field is missing.
fn default_log_file() -> String {
    String::from("getquotes.log")
}

/// Default list of authors (used if config file is missing).
fn default_authors() -> Vec<String> {
    vec![
        "Mahatma Gandhi".into(),
        "Albert Einstein".into(),
        "Martin Luther King, Jr.".into(),
        "Leonardo da Vinci".into(),
        "Walt Disney".into(),
        "Edgar Allan Poe".into(),
        "Sigmund Freud".into(),
        "Thomas A. Edison".into(),
        "Robin Williams".into(),
        "Steve Jobs".into(),
    ]
}

/// Load or create config file at ~/.config/getquotes/config.json
/// If the file does not exist, creates it with a default list of authors, theme_color, max_tries, and log_file.
fn load_or_create_config() -> Result<Config, Box<dyn Error>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        if let Some(parent_dir) = config_path.parent() {
            create_dir_all(parent_dir)?;
        }
        // Create a default config if not found
        let default_config = Config {
            authors: default_authors(),
            theme_color: default_theme_color(),
            max_tries: default_max_tries(),
            log_file: default_log_file(),
        };
        let file = File::create(&config_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer_pretty(writer, &default_config)?;
        info!("Config file created at: {:?}", config_path);
        return Ok(default_config);
    }

    let file = File::open(&config_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;
    info!("Config file loaded from: {:?}", config_path);
    Ok(config)
}

/// Return the path ~/.config/getquotes/config.json
fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
    let home = home_dir().ok_or("Unable to find home directory.")?;
    let config_path = home.join(".config/getquotes/config.json");
    Ok(config_path)
}

/// Initialize the logger with both console and file outputs.
fn initialize_logger(log_file: &str) -> Result<(), Box<dyn Error>> {
    let log_path = home_dir()
        .ok_or("Unable to find home directory for log file.")?
        .join(".config/getquotes")
        .join(log_file);

    // Ensure the log directory exists
    if let Some(parent_dir) = log_path.parent() {
        create_dir_all(parent_dir)?;
    }

    // Open the log file in append mode
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    // Configure the logger to write to both stdout and file
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(file)))
        .try_init()?;

    Ok(())
}

/// Represents the JSON structure of a Wikiquote "parse" response.
/// We only care about sections and text to extract quotes.
#[derive(Deserialize, Debug)]
struct QueryResult {
    parse: ParseResult,
}

#[derive(Deserialize, Debug)]
struct ParseResult {
    title: String,
    sections: Option<Vec<Section>>,
    #[serde(default)]
    text: Option<Text>,
}

#[derive(Deserialize, Debug)]
struct Section {
    index: String,
    number: String,
    line: String,
}

#[derive(Deserialize, Debug)]
struct Text {
    #[serde(rename = "*")]
    content: String,
}

/// Parse a hex code string of form "#RRGGBB" or "RRGGBB" into (r, g, b) components.
fn parse_hex_color(hex_str: &str) -> Option<(u8, u8, u8)> {
    // Remove leading '#' if present
    let clean_hex = if let Some(stripped) = hex_str.strip_prefix('#') {
        stripped
    } else {
        hex_str
    };

    // Must be exactly 6 characters for RRGGBB
    if clean_hex.len() != 6 {
        return None;
    }

    // Parse the components from the string
    let r = u8::from_str_radix(&clean_hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean_hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean_hex[4..6], 16).ok()?;
    Some((r, g, b))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load or create config file with authors, theme_color, max_tries, and log_file
    let cfg = load_or_create_config()?;

    // Initialize the logger
    initialize_logger(&cfg.log_file)?;
    info!("Logger initialized. Log file: {}", cfg.log_file);

    debug!("Loaded config: {:?}", cfg);

    // Parse the theme_color from the config
    let (r, g, b) = match parse_hex_color(&cfg.theme_color) {
        Some((r, g, b)) => {
            debug!(
                "Parsed theme_color from config: #{:02X}{:02X}{:02X}",
                r, g, b
            );
            (r, g, b)
        }
        None => {
            warn!(
                "Invalid hex code in config: '{}'. Using fallback #1E90FF.",
                cfg.theme_color
            );
            // Fallback if parsing fails
            (0x1E, 0x90, 0xFF)
        }
    };

    // Create an HTTP client
    let client = Client::new();
    info!("HTTP client initialized.");

    // Attempt up to max_tries to find a quote
    let max_tries = cfg.max_tries;
    let mut rng = thread_rng();

    for attempt in 1..=max_tries {
        debug!("Attempt {}/{}", attempt, max_tries);
        // Pick a random author from config
        let author_idx = rng.gen_range(0..cfg.authors.len());
        let author = &cfg.authors[author_idx];

        info!("Attempting to fetch quote for author: {}", author);

        // Get the page sections for the chosen author
        match get_author_sections(&client, author).await {
            Ok(Some((title, sections))) => {
                if !sections.is_empty() {
                    let mut found_quote = None;
                    for section in sections {
                        debug!("Fetching quotes from section: {}", section.line);
                        let quotes = match fetch_quotes(&client, &title, &section.index).await {
                            Ok(q) => q,
                            Err(err) => {
                                error!("Failed to fetch quotes for author {}: {}", author, err);
                                continue;
                            }
                        };

                        if !quotes.is_empty() {
                            let random_quote = &quotes[rng.gen_range(0..quotes.len())];
                            found_quote = Some((author.to_string(), random_quote.clone()));
                            break;
                        }
                    }
                    if let Some((auth_found, quote_found)) = found_quote {
                        // Apply the parsed hex color to the quote
                        let colorized_quote = format!("“{}”", quote_found).truecolor(r, g, b);
                        let dash = "-";

                        // Print the quote and the author on a new line, right-aligned
                        println!(
                            "{}\n\n {:>99}{}",
                            colorized_quote.bold(),
                            dash.bold().green(),
                            auth_found.green()
                        );
                        info!("Quote successfully displayed from author: {}", auth_found);
                        return Ok(());
                    } else {
                        warn!(
                            "No suitable quotes found for author '{}', trying again.",
                            author
                        );
                    }
                } else {
                    warn!("No sections found for author '{}', trying again.", author);
                }
            }
            Ok(None) => {
                warn!("No valid page found for author '{}', trying again.", author);
            }
            Err(err) => {
                error!("Failed to get sections for author '{}': {}", author, err);
            }
        }
    }

    error!(
        "Could not find a suitable quote after {} attempts.",
        max_tries
    );
    Err("Failed to retrieve a quote.".into())
}

/// Retrieve sections for the specified author page from Wikiquote.
async fn get_author_sections(
    client: &Client,
    author: &str,
) -> Result<Option<(String, Vec<Section>)>, Box<dyn Error>> {
    let author_encoded = urlencoding::encode(author);
    let api_url = format!(
        "https://en.wikiquote.org/w/api.php?action=parse&format=json&prop=sections&page={}",
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

/// Fetch quotes for given title & section index, returning raw text from <li> tags.
async fn fetch_quotes(
    client: &Client,
    title: &str,
    section: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let title_encoded = urlencoding::encode(title);
    let api_url = format!(
        "https://en.wikiquote.org/w/api.php?action=parse&format=json&prop=text&page={}&section={}",
        title_encoded, section
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

/// Below are unit tests that verify our helper functions and configurations
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color_valid() {
        let color = parse_hex_color("#1E90FF");
        assert_eq!(color, Some((0x1E, 0x90, 0xFF)));
    }

    #[test]
    fn test_parse_hex_color_missing_hash() {
        let color = parse_hex_color("1E90FF");
        assert_eq!(color, Some((0x1E, 0x90, 0xFF)));
    }

    #[test]
    fn test_parse_hex_color_invalid_length() {
        let color = parse_hex_color("#FFF");
        assert_eq!(color, None);
    }

    #[test]
    fn test_parse_hex_color_invalid_chars() {
        // Contains invalid hex chars G, H
        let color = parse_hex_color("#GGHHII");
        assert_eq!(color, None);
    }

    #[test]
    fn test_default_theme_color() {
        let default_color = default_theme_color();
        assert_eq!(default_color, "#20B2AA");
    }

    #[test]
    fn test_default_max_tries() {
        let default_max = default_max_tries();
        assert_eq!(default_max, 30);
    }

    #[test]
    fn test_default_log_file() {
        let default_log = default_log_file();
        assert_eq!(default_log, "getquotes.log");
    }

    #[test]
    fn test_load_or_create_config() {
        // This test assumes that the config file does not exist.
        // In a real-world scenario, you'd mock the filesystem.
        let result = load_or_create_config();
        assert!(result.is_ok());
        let config = result.unwrap();
        assert_eq!(config.authors.len(), 10);
        assert_eq!(config.theme_color, "#20B2AA");
        assert_eq!(config.max_tries, 30);
        assert_eq!(config.log_file, "getquotes.log");
    }

    #[tokio::test]
    async fn test_get_author_sections_invalid_author() {
        let client = Client::new();
        let result = get_author_sections(&client, "NonExistentAuthor12345").await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_fetch_quotes_invalid_section() {
        let client = Client::new();
        let result = fetch_quotes(&client, "Albert Einstein", "999").await;
        assert!(result.is_ok());
        let quotes = result.unwrap();
        assert!(quotes.is_empty());
    }

    #[tokio::test]
    async fn test_fetch_quotes_valid() {
        let client = Client::new();
        let quotes = fetch_quotes(&client, "Albert Einstein", "1").await;
        assert!(quotes.is_ok());
        let quotes = quotes.unwrap();
        assert!(!quotes.is_empty());
    }
}
