use std::error::Error;
use std::sync::{Mutex, MutexGuard, OnceLock};
use tempfile::TempDir;

static HOME_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();

fn home_mutex() -> &'static Mutex<()> {
    HOME_MUTEX.get_or_init(|| Mutex::new(()))
}

pub fn setup_temp_home() -> Result<(MutexGuard<'static, ()>, TempDir), Box<dyn Error + Send + Sync>> {
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
