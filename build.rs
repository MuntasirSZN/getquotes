use clap::{CommandFactory, ValueEnum};
use clap_complete::generate_to;
use std::fs;
use std::path::Path;

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    // Define the output directory for completion scripts
    let out_dir = Path::new("completions");
    fs::create_dir_all(out_dir)?;

    // Generate completion scripts for each supported shell
    for shell in Shell::value_variants() {
        let mut cmd = Args::command();
        generate_to(*shell, &mut cmd, "getquotes", out_dir)?;
    }

    Ok(())
}
