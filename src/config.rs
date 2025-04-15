use log::info;
use serde::{Deserialize, Serialize};
use std::env::home_dir;
use std::error::Error as StdError;
use std::fs::{File, create_dir_all, read_to_string, write};
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub authors: Vec<String>,
    #[serde(default = "default_theme_color")]
    pub theme_color: String,
    #[serde(default = "default_max_tries")]
    pub max_tries: usize,
    #[serde(default = "default_log_file")]
    pub log_file: String,
    #[serde(default = "default_rainbow_mode")]
    pub rainbow_mode: bool,
}

pub fn default_theme_color() -> String {
    "#B7FFFA".to_string()
}

pub fn default_max_tries() -> usize {
    30
}

pub fn default_log_file() -> String {
    String::from("getquotes.log")
}

pub fn default_authors() -> Vec<String> {
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

pub fn load_or_create_config() -> Result<Config, Box<dyn StdError + Send + Sync>> {
    let config_path = get_config_path()?;
    if !config_path.exists() {
        if let Some(parent_dir) = config_path.parent() {
            create_dir_all(parent_dir)?;
        }
        let default_config = Config {
            authors: default_authors(),
            theme_color: default_theme_color(),
            max_tries: default_max_tries(),
            log_file: default_log_file(),
            rainbow_mode: default_rainbow_mode(),
        };
        let toml_string = toml::to_string_pretty(&default_config)?;
        write(&config_path, toml_string)?;
        info!("Config file created at: {:?}", config_path);
        return Ok(default_config);
    }

    let toml_content = read_to_string(&config_path)?;
    let config: Config = toml::from_str(&toml_content)?;
    info!("Config file loaded from: {:?}", config_path);
    Ok(config)
}

pub fn get_config_path() -> Result<PathBuf, Box<dyn StdError + Send + Sync>> {
    let home = home_dir()
        .ok_or_else(|| Box::<dyn StdError + Send + Sync>::from("Unable to find home directory."))?;
    let config_dir = home.join(".config/getquotes");
    create_dir_all(&config_dir)?;
    let config_path = config_dir.join("config.toml");
    Ok(config_path)
}

pub fn load_or_create_config_from_path(
    path: &str,
) -> Result<Config, Box<dyn StdError + Send + Sync>> {
    let config_path = PathBuf::from(path);
    if !config_path.exists() {
        if let Some(parent_dir) = config_path.parent() {
            create_dir_all(parent_dir)?;
        }
        let default_config = Config {
            authors: default_authors(),
            theme_color: default_theme_color(),
            max_tries: default_max_tries(),
            log_file: default_log_file(),
            rainbow_mode: default_rainbow_mode(),
        };
        let toml_string = toml::to_string_pretty(&default_config)?;
        write(&config_path, toml_string)?;
        info!("Config file created at: {:?}", config_path);
        return Ok(default_config);
    }

    let toml_content = read_to_string(&config_path)?;
    let config: Config = toml::from_str(&toml_content)?;
    info!("Config file loaded from: {:?}", config_path);
    Ok(config)
}

pub fn parse_hex_color(hex_str: &str) -> Option<(u8, u8, u8)> {
    let clean_hex = hex_str.strip_prefix('#').unwrap_or(hex_str);
    if clean_hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&clean_hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&clean_hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&clean_hex[4..6], 16).ok()?;
    Some((r, g, b))
}

pub fn default_rainbow_mode() -> bool {
    false
}

pub fn migrate_json_to_toml() -> Result<(), Box<dyn StdError + Send + Sync>> {
    // Determine the JSON config path
    let home = home_dir()
        .ok_or_else(|| Box::<dyn StdError + Send + Sync>::from("Unable to find home directory."))?;
    let config_dir = home.join(".config/getquotes");
    let json_config_path = config_dir.join("config.json");

    // Check if JSON config exists
    if !json_config_path.exists() {
        return Err("JSON config file not found. Nothing to migrate.".into());
    }

    // Read JSON config
    let file = File::open(&json_config_path)?;
    let reader = BufReader::new(file);
    let config: Config = serde_json::from_reader(reader)?;

    // Write as TOML
    let config_path = get_config_path()?;
    let toml_string = toml::to_string_pretty(&config)?;
    write(&config_path, toml_string)?;

    info!("Config migrated from JSON to TOML: {:?}", config_path);

    // Don't delete the old JSON file - let the user do that manually if they wish
    info!(
        "Migration complete. The original JSON config file still exists at: {:?}",
        json_config_path
    );

    Ok(())
}
