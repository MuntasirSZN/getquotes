mod common;

use getquotes::{run, Args};
use tokio;

#[tokio::test]
async fn test_full_quote_retrieval() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _temp = common::setup_temp_home()?;
    
    let args = Args {
        authors: Some("Einstein".to_string()),
        theme_color: Some("#FF0000".to_string()),
        max_tries: Some(5),
        log_file: Some("test.log".to_string()),
        rainbow_mode: false,
        init_cache: false,
        offline: false,
    };

    let result = run(args).await;
    assert!(result.is_ok());
    Ok(())
}
