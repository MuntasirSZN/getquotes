use dirs::home_dir;
use std::error::Error as StdError;
use std::fs::{OpenOptions, create_dir_all};

pub fn initialize_logger(log_file: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
    let home = home_dir()
        .ok_or_else(|| Box::<dyn StdError + Send + Sync>::from("Unable to find home directory."))?;
    let log_dir = home.join(".config/getquotes");
    create_dir_all(&log_dir)?;
    let log_path = log_dir.join(log_file);

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
