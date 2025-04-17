mod common;

use getquotes::cache::init_cache;
use getquotes::cli::Args;
use getquotes::config::Config;
use getquotes::run;
use std::fs::{self, write};
use std::path::PathBuf;
use tokio::runtime::Runtime;

#[test]
fn test_run_with_version_flag() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: false,
        init_cache: false,
        offline: false,
        version: true,
        config: None,
        completion: None,
        migrate_config: false,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_migrate_config() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;
    let home = std::env::home_dir();
    let config_dir = home
        .map(|path| path.join(".config/getquotes"))
        .unwrap_or_else(|| PathBuf::from("~/.config/getquotes"));
    let json_config_path = config_dir.join("config.json");

    fs::create_dir_all(&config_dir)?;

    let json_config = r#"{
        "authors": ["JSON Author"],
        "theme_color": "112233",
        "max_tries": 15,
        "log_file": "json.log ",
        "rainbow_mode": true
    }"#;

    fs::write(&json_config_path, json_config)?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: false,
        init_cache: false,
        offline: false,
        version: false,
        config: None,
        completion: None,
        migrate_config: true,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    let toml_config_path = config_dir.join("config.toml");
    assert!(toml_config_path.exists());

    Ok(())
}

#[test]
fn test_run_with_completion() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: false,
        init_cache: false,
        offline: false,
        version: false,
        config: None,
        completion: Some(getquotes::cli::Shell::Bash),
        migrate_config: false,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_offline_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    init_cache()?;
    let db_path = getquotes::cache::get_database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    conn.execute(
        "INSERT INTO quotes (author, quote) VALUES (?1, ?2)",
        ["Test Author", "This is a test quote for offline mode."],
    )?;

    let config_path = getquotes::config::get_config_path()?;
    let config = Config {
        authors: vec!["Test Author".to_string()],
        theme_color: getquotes::config::default_theme_color(),
        max_tries: getquotes::config::default_max_tries(),
        log_file: getquotes::config::default_log_file(),
        rainbow_mode: getquotes::config::default_rainbow_mode(),
    };
    let toml_string = toml::to_string_pretty(&config)?;
    write(&config_path, toml_string)?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: false,
        init_cache: false,
        offline: true,
        version: false,
        config: None,
        completion: None,
        migrate_config: false,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_custom_config() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = common::setup_temp_home()?;

    let custom_config_path = temp_dir.path().join("custom_config.toml");
    let custom_config = r#"
authors = ["Custom Config Author"]
theme_color = "AABBCC"
max_tries = 5
log_file = "custom_config.log"
rainbow_mode = true
    "#;
    fs::write(&custom_config_path, custom_config)?;

    init_cache()?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: false,
        init_cache: true,
        offline: true,
        version: false,
        config: Some(custom_config_path.to_str().unwrap().to_string()),
        completion: None,
        migrate_config: false,
    };

    // Insert test quote for the offline mode
    let db_path = getquotes::cache::get_database_path()?;
    let conn = rusqlite::Connection::open(db_path)?;
    conn.execute(
        "INSERT OR IGNORE INTO quotes (author, quote) VALUES (?1, ?2)",
        [
            "Custom Config Author",
            "Custom config quote for offline testing",
        ],
    )?;

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}
