mod common;

use getquotes::config::{load_or_create_config, parse_hex_color};

#[test]
pub fn test_hex_color_parsing() {
    assert_eq!(parse_hex_color("#FF0000"), Some((255, 0, 0)));
    assert_eq!(parse_hex_color("FF0000"), Some((255, 0, 0)));
    assert_eq!(parse_hex_color("#GGGGGG"), None);
    assert_eq!(parse_hex_color("#FF00"), None);
}

#[test]
pub fn test_config_creation_and_loading() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _temp = common::setup_temp_home()?;

    // First load should create default config
    let config = load_or_create_config()?;
    assert_eq!(config.theme_color, "#B7FFFA");
    assert_eq!(config.max_tries, 30);

    // Second load should read the same config
    let config2 = load_or_create_config()?;
    assert_eq!(config2.theme_color, config.theme_color);
    Ok(())
}
