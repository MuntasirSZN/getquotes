mod common;

use getquotes::config::{
    default_authors, default_log_file, default_max_tries, default_rainbow_mode,
    default_theme_color, get_config_path, load_or_create_config, load_or_create_config_from_path,
    migrate_json_to_toml, parse_hex_color,
};
use std::fs::{self};
use std::path::PathBuf;
use tempfile::TempDir;

#[test]
fn test_default_config_values() {
    let authors = default_authors();
    assert!(!authors.is_empty());
    assert!(authors.contains(&"Albert Einstein".to_string()));

    let theme_color = default_theme_color();
    assert_eq!(theme_color, "#B7FFFA");

    let max_tries = default_max_tries();
    assert_eq!(max_tries, 30);

    let log_file = default_log_file();
    assert_eq!(log_file, "getquotes.log");

    let rainbow_mode = default_rainbow_mode();
    assert!(!rainbow_mode);
}

#[test]
fn test_hex_color_parsing_valid_formats() {
    // Test with hash
    assert_eq!(parse_hex_color("#FF0000"), Some((255, 0, 0)));

    // Test without hash
    assert_eq!(parse_hex_color("FF0000"), Some((255, 0, 0)));

    // Test lowercase
    assert_eq!(parse_hex_color("#ff0000"), Some((255, 0, 0)));

    // Test mixed case
    assert_eq!(parse_hex_color("#Ff00Dd"), Some((255, 0, 221)));
}

#[test]
fn test_hex_color_parsing_invalid_formats() {
    // Test invalid characters
    assert_eq!(parse_hex_color("#GGGGGG"), None);

    // Test too short
    assert_eq!(parse_hex_color("#FF00"), None);

    // Test too long
    assert_eq!(parse_hex_color("#FF00000"), None);

    // Test empty string
    assert_eq!(parse_hex_color(""), None);
}

#[test]
fn test_get_config_path() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    let config_path = get_config_path()?;

    assert!(config_path.ends_with(".config/getquotes/config.toml"));

    let parent = config_path.parent().unwrap();
    assert!(parent.exists());

    Ok(())
}

#[test]
fn test_load_or_create_config_new_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    // Get the config path and make sure it doesn't exist
    let config_path = get_config_path()?;
    assert!(!config_path.exists());

    // Load the config, which should create a new one
    let config = load_or_create_config()?;

    // Verify default values
    assert_eq!(config.theme_color, default_theme_color());
    assert_eq!(config.max_tries, default_max_tries());
    assert_eq!(config.log_file, default_log_file());
    assert!(!config.rainbow_mode);
    assert_eq!(config.authors, default_authors());

    // Verify file was created
    assert!(config_path.exists());

    // Read the file and check the content
    let content = fs::read_to_string(&config_path)?;
    let parsed_config: getquotes::config::Config = toml::from_str(&content)?;

    assert_eq!(parsed_config.theme_color, config.theme_color);

    Ok(())
}

#[test]
fn test_load_or_create_config_existing_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    common::setup_temp_home()?;

    // Get the config path and create a custom config
    let config_path = get_config_path()?;

    // Create a parent directory
    if let Some(parent) = config_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Create custom config contents
    let custom_config = r#"
        authors = ["Custom Author"]
        theme_color = "FF5500"
        max_tries = 50
        log_file = "custom.log"
        rainbow_mode = true
    "#;

    // Write the custom config
    fs::write(&config_path, custom_config)?;

    // Load the config
    let config = load_or_create_config()?;

    // Verify custom values
    assert_eq!(config.theme_color, "FF5500");
    assert_eq!(config.max_tries, 50);
    assert_eq!(config.log_file, "custom.log");
    assert!(config.rainbow_mode);
    assert_eq!(config.authors, vec!["Custom Author"]);

    Ok(())
}

#[test]
fn test_load_or_create_config_from_path() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = TempDir::new()?;
    let custom_path = temp_dir.path().join("custom_config.toml");

    // The file doesn't exist yet
    assert!(!custom_path.exists());

    // Load/create config from the custom path
    let config = load_or_create_config_from_path(custom_path.to_str().unwrap())?;

    // Verify the file was created with default values
    assert!(custom_path.exists());
    assert_eq!(config.theme_color, default_theme_color());

    // Now modify the file and reload
    let custom_config = r#"
        authors = ["Custom Author From Path"]
        theme_color = "AABBCC"
        max_tries = 99
        log_file = "path_custom.log"
        rainbow_mode = true
    "#;

    fs::write(&custom_path, custom_config)?;

    // Reload the config
    let reloaded_config = load_or_create_config_from_path(custom_path.to_str().unwrap())?;

    // Verify the modified values
    assert_eq!(reloaded_config.theme_color, "AABBCC");
    assert_eq!(reloaded_config.max_tries, 99);
    assert_eq!(reloaded_config.log_file, "path_custom.log");
    assert!(reloaded_config.rainbow_mode);
    assert_eq!(reloaded_config.authors, vec!["Custom Author From Path"]);

    Ok(())
}

#[test]
fn test_migrate_json_to_toml() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;
    let home = std::env::home_dir();
    let config_dir = home
        .map(|path| path.join(".config/getquotes"))
        .unwrap_or_else(|| PathBuf::from("~/.config/getquotes"));
    let json_config_path = config_dir.join("config.json");
    let toml_config_path = config_dir.join("config.toml");

    // Create directory structure
    fs::create_dir_all(&config_dir)?;

    // Create a JSON config
    let json_config = r#"{
        "authors": ["JSON Author"],
        "theme_color": "112233",
        "max_tries": 15,
        "log_file": "json.log",
        "rainbow_mode": true
    }"#;

    fs::write(&json_config_path, json_config)?;
    assert!(json_config_path.exists());

    // Run the migration
    migrate_json_to_toml()?;

    // Verify TOML file was created
    assert!(toml_config_path.exists());

    // Verify the JSON file still exists
    assert!(json_config_path.exists());

    // Read and verify the TOML content
    let toml_content = fs::read_to_string(&toml_config_path)?;
    let config: getquotes::config::Config = toml::from_str(&toml_content)?;

    assert_eq!(config.theme_color, "112233");
    assert_eq!(config.max_tries, 15);
    assert_eq!(config.log_file, "json.log");
    assert!(config.rainbow_mode);
    assert_eq!(config.authors, vec!["JSON Author"]);

    Ok(())
}

#[test]
fn test_migrate_json_to_toml_missing_file() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
{
    common::setup_temp_home()?;

    // Don't create a JSON file

    // Try to migrate - should fail
    let result = migrate_json_to_toml();
    assert!(result.is_err());

    Ok(())
}
