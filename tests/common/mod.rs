use std::error::Error;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tempfile::TempDir;

static HOME_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static API_URL_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn home_mutex() -> &'static Mutex<()> {
    HOME_MUTEX.get_or_init(|| Mutex::new(()))
}

fn api_url_mutex() -> &'static Mutex<()> {
    API_URL_MUTEX.get_or_init(|| Mutex::new(()))
}

pub fn setup_temp_home() -> Result<(MutexGuard<'static, ()>, TempDir), Box<dyn Error + Send + Sync>>
{
    let guard = home_mutex().lock().unwrap_or_else(|e| e.into_inner());
    let temp_dir = TempDir::new()?;

    #[cfg(windows)]
    unsafe {
        std::env::set_var("USERPROFILE", temp_dir.path().to_str().unwrap());
    }

    #[cfg(not(windows))]
    unsafe {
        std::env::set_var("HOME", temp_dir.path().to_str().unwrap());
    }
    Ok((guard, temp_dir))
}

pub fn setup_api_url(url: &str) -> MutexGuard<'static, ()> {
    let guard = api_url_mutex().lock().unwrap_or_else(|e| e.into_inner());
    unsafe { std::env::set_var("WIKIQUOTE_API_URL", url) };
    guard
}
