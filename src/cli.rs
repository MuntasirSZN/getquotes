use clap::Parser;
use clap_complete::Shell;

#[derive(Parser, Debug)]
#[command(
    about,
    author,
    long_about = None,
    help_template("\
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}

{author-with-newline}
"))]
pub struct Args {
    #[arg(long, short, help = "Specify a list of authors to fetch quotes from")]
    pub authors: Option<String>,

    #[arg(long, short, help = "Set the theme color for the displayed quotes")]
    pub theme_color: Option<String>,

    #[arg(long, short, help = "Set the maximum number of tries to fetch a quote")]
    pub max_tries: Option<usize>,

    #[arg(long, short, help = "Specify the log file path")]
    pub log_file: Option<String>,

    #[arg(long, short, help = "Enable rainbow mode for random quote colors")]
    pub rainbow_mode: bool,

    #[arg(long, short, help = "Initialize the quote cache for offline mode")]
    pub init_cache: bool,

    #[arg(long, short, help = "Run in offline mode, using cached quotes")]
    pub offline: bool,

    #[arg(long, short, help = "Print version information")]
    pub version: bool,

    #[arg(long, short = 'C', help = "Use a custom TOML configuration file")]
    pub config: Option<String>,

    #[arg(long, short, help = "Generate shell completion script")]
    pub completion: Option<Shell>,

    #[arg(
        long,
        short = 'M',
        help = "Migrate JSON config to TOML format (will be removed in next major release)"
    )]
    pub migrate_config: bool,
}
