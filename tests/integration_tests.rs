mod common;

use getquotes::{run, Args};
use std::error::Error as StdError;
use tempdir::TempDir;

#[tokio::test]
async fn test_full_quote_retrieval() -> Result<(), Box<dyn StdError + Send + Sync>> {
    let temp_dir = TempDir::new("test")?;
    let args = Args {
        authors: Some("Einstein".to_string()),
        theme_color: Some("#FF0000".to_string()),
        max_tries: Some(5),
        log_file: Some(temp_dir.path().join("test.log").to_str().unwrap().to_string()),
        rainbow_mode: false,
        init_cache: false,
        offline: false,
        version: false,
    };

    let result = run(args).await;
    assert!(result.is_ok());
    Ok(())
}
