mod common;

use getquotes::cache::init_cache;
use getquotes::cli::Args;
use getquotes::config::Config;
use getquotes::run;
use std::fs::{self, write};
use tokio::runtime::Runtime;

#[test]
fn test_run_with_version_flag() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_guard, _temp_dir) = common::setup_temp_home()?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: None,
        init_cache: None,
        offline: false,
        version: true,
        config: None,
        completion: None,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_completion() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_guard, _temp_dir) = common::setup_temp_home()?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: None,
        init_cache: None,
        offline: false,
        version: false,
        config: None,
        completion: Some(getquotes::cli::Shell::Bash),
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_offline_mode() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_guard, _temp_dir) = common::setup_temp_home()?;

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
        quote_style: getquotes::config::default_quote_style(),
        author_style: getquotes::config::default_author_style(),
        nested_quote_style: getquotes::config::default_nested_quote_style(),
        max_tries: getquotes::config::default_max_tries(),
        log_file: getquotes::config::default_log_file(),
        rainbow_mode: getquotes::config::default_rainbow_mode(),
        layout: getquotes::config::default_layout(),
        box_corners: getquotes::config::default_box_corners(),
        prefer_cache: getquotes::config::default_prefer_cache(),
        api_calls_per_minute: getquotes::config::default_api_calls_per_minute(),
    };
    let toml_string = toml::to_string_pretty(&config)?;
    write(&config_path, toml_string)?;

    let rt = Runtime::new()?;

    let args = Args {
        authors: None,
        theme_color: None,
        max_tries: None,
        log_file: None,
        rainbow_mode: None,
        init_cache: None,
        offline: true,
        version: false,
        config: None,
        completion: None,
    };

    let result = rt.block_on(run(args));

    assert!(result.is_ok());

    Ok(())
}

#[test]
fn test_run_with_custom_config() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (_guard, temp_dir) = common::setup_temp_home()?;

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
        rainbow_mode: None,
        init_cache: Some("".to_string()),
        offline: true,
        version: false,
        config: Some(custom_config_path.to_str().unwrap().to_string()),
        completion: None,
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
