mod common;

use common::setup_temp_home;
use getquotes::logger::initialize_logger;
use std::fs;
use std::fs::create_dir_all;

#[test]
pub fn test_logger_init() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp = setup_temp_home()?;
    let log_dir = temp.path().join(".config/getquotes");
    create_dir_all(&log_dir)?;
    let log_path = log_dir.join("test.log");

    // Initialize logger with the test log file
    initialize_logger(log_path.to_str().unwrap())?;
    log::info!("Test log message");

    // Wait for logging to complete
    std::thread::sleep(std::time::Duration::from_millis(100));

    let content = fs::read_to_string(&log_path)?;
    assert!(content.contains("Test log message"));
    Ok(())
}
