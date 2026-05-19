use getquotes::config::Config;
use schemars::schema_for;
use std::error::Error;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let schema = schema_for!(Config);
    let output_dir = Path::new("config");
    fs::create_dir_all(output_dir)?;
    let output_path = output_dir.join("config.schema.json");
    fs::write(output_path, serde_json::to_string_pretty(&schema)?)?;
    Ok(())
}
