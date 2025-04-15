mod common;

use getquotes::logger::initialize_logger;
use std::fs;
use std::path::Path;

#[test]
fn test_initialize_logger() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let temp_dir = common::setup_temp_home()?;

    // Make sure we're using a unique log file for this test
    let log_file = format!("test_logger_{}.log", std::process::id());

    initialize_logger(&log_file)?;

    let home = std::env::home_dir();
    let expected_log_path = home.as_ref().map_or_else(
        || Path::new("").join(&log_file),
        |h| Path::new(h).join(".config/getquotes").join(&log_file),
    );

    assert!(
        expected_log_path.exists(),
        "Log file was not created at expected path: {:?}",
        expected_log_path
    );

    // Write multiple log entries to ensure something gets written
    log::error!("Test error log entry");
    log::warn!("Test warning log entry");
    log::info!("Test info log entry");

    // Give the logger more time to flush content to the file
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Force flush any buffered content
    log::logger().flush();

    // Wait again to ensure file system writes complete
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Read the log content
    let log_content = fs::read_to_string(&expected_log_path)?;

    // Check if log file contains content
    assert!(
        !log_content.is_empty(),
        "Log file is empty after logging entries"
    );

    // Make sure the temporary directory stays alive until the end of the test
    std::mem::drop(temp_dir);

    Ok(())
}

#[test]
fn test_initialize_logger_custom_dir() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    common::setup_temp_home()?;

    let log_file = "custom/dir/test_logger.log";

    let result = initialize_logger(log_file);
    assert!(result.is_err());

    Ok(())
}
