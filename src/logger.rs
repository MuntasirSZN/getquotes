use std::env::home_dir;
use std::error::Error as StdError;
use std::fs::{OpenOptions, create_dir_all};
use std::io::{self, Write};

pub fn initialize_logger(log_file: &str) -> Result<(), Box<dyn StdError + Send + Sync>> {
    // Check if log_file contains path separators, which isn't allowed
    if log_file.contains('/') {
        return Err(Box::<dyn StdError + Send + Sync>::from(
            "Log file path cannot contain directories.",
        ));
    }

    let home = home_dir()
        .ok_or_else(|| Box::<dyn StdError + Send + Sync>::from("Unable to find home directory."))?;
    let log_dir = home.join(".config/getquotes");
    create_dir_all(&log_dir)?;
    let log_path = log_dir.join(log_file);

    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;

    // Wrap the file in a LineWriter for more efficient writes
    let file = io::LineWriter::new(file);

    // Create a new logger or get the existing one
    let logger = env_logger::Builder::from_default_env()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                jiff::Zoned::now().strftime("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Pipe(Box::new(file)))
        .build();

    // Set the logger
    log::set_boxed_logger(Box::new(logger))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .map_err(|e| Box::<dyn StdError + Send + Sync>::from(e.to_string()))?;

    // Write an initialization message to ensure file has content
    log::info!("Logger initialized");

    // Flush the logger to ensure content is written
    log::logger().flush();

    Ok(())
}
