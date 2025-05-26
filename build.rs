use clap::CommandFactory;
use clap_complete::generate_to;
use std::fs;

include!("src/cli.rs");

fn main() -> std::io::Result<()> {
    let mut cmd = Args::command();
    let completion_out_dir = Path::new("completions");

    fs::create_dir_all(completion_out_dir)?;

    for shell in Shell::value_variants() {
        generate_to(*shell, &mut cmd, "getquotes", completion_out_dir)?;
    }
    let man_out_dir = Path::new("man");

    fs::create_dir_all(man_out_dir)?;

    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    std::fs::write(man_out_dir.join("getquotes.1"), buffer)?;

    Ok(())
}
