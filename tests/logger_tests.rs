mod common;

use getquotes::logger::initialize_logger;
use std::fs;

#[test]
pub fn test_logger_init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp = common::setup_temp_home()?;
    let log_path = temp.path().join("test.log");
    
    initialize_logger(log_path.to_str().unwrap())?;
    log::info!("Test log message");
    
    let content = fs::read_to_string(log_path)?;
    assert!(content.contains("Test log message"));
    Ok(())
}
