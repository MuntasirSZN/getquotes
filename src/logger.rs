use dirs::home_dir;
use std::error::Error as StdError;
use std::fs::{create_dir_all, OpenOptions};

pub fn initialize_logger(log_file: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let log_path = home_dir()
        .ok_or_else(|| {
            Box::<dyn StdError + Send + Sync>::from("Unable to find home directory for log file.")
        })?
        .join(".config/getquotes")
        .join(log_file);

    if let Some(parent_dir) = log_path.parent() {
        create_dir_all(parent_dir)?;
    }

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            use std::io::Write;
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(file)))
        .init();

    Ok(())
}
