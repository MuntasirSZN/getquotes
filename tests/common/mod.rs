use std::error::Error;
use tempdir::TempDir;

pub fn setup_temp_home() -> Result<TempDir, Box<dyn Error + Send + Sync>> {
    let temp_dir = TempDir::new("getquotes_test")?;

    #[cfg(windows)]
    unsafe {
        std::env::set_var("USERPROFILE", temp_dir.path().to_str().unwrap());
    }

    #[cfg(not(windows))]
    unsafe {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    Ok(temp_dir)
}
