use clap::{CommandFactory, Parser};
use getquotes::cli::Args;
use getquotes::cli::Shell;

#[test]
fn test_cli_arguments() {
    // Test default values
    let default_args = Args::parse_from(["getquotes"]);
    assert_eq!(default_args.authors, None);
    assert_eq!(default_args.theme_color, None);
    assert_eq!(default_args.max_tries, None);
    assert_eq!(default_args.log_file, None);
    assert!(!default_args.rainbow_mode);
    assert!(!default_args.init_cache);
    assert!(!default_args.offline);
    assert!(!default_args.version);
    assert_eq!(default_args.config, None);
    assert_eq!(default_args.completion, None);
    assert!(!default_args.migrate_config);

    // Test setting authors
    let args = Args::parse_from(["getquotes", "--authors", "Author1,Author2"]);
    assert_eq!(args.authors, Some("Author1,Author2".to_string()));

    // Test setting theme color
    let args = Args::parse_from(["getquotes", "--theme-color", "#FF00FF"]);
    assert_eq!(args.theme_color, Some("#FF00FF".to_string()));

    // Test setting max tries
    let args = Args::parse_from(["getquotes", "--max-tries", "42"]);
    assert_eq!(args.max_tries, Some(42));

    // Test setting log file
    let args = Args::parse_from(["getquotes", "--log-file", "custom.log"]);
    assert_eq!(args.log_file, Some("custom.log".to_string()));

    // Test setting rainbow mode
    let args = Args::parse_from(["getquotes", "--rainbow-mode"]);
    assert!(args.rainbow_mode);

    // Test setting init cache
    let args = Args::parse_from(["getquotes", "--init-cache"]);
    assert!(args.init_cache);

    // Test setting offline mode
    let args = Args::parse_from(["getquotes", "--offline"]);
    assert!(args.offline);

    // Test setting version flag
    let args = Args::parse_from(["getquotes", "--version"]);
    assert!(args.version);

    // Test setting config
    let args = Args::parse_from(["getquotes", "--config", "custom.toml"]);
    assert_eq!(args.config, Some("custom.toml".to_string()));

    // Test setting completion
    let args = Args::parse_from(["getquotes", "--completion", "bash"]);
    if let Some(shell) = args.completion {
        match shell {
            Shell::Bash => (), // This is expected
            _ => panic!("Expected Shell::Bash"),
        }
    } else {
        panic!("Expected Some(Shell::Bash)");
    }

    // Test setting migrate config
    let args = Args::parse_from(["getquotes", "--migrate-config"]);
    assert!(args.migrate_config);

    // Test short options
    let args = Args::parse_from([
        "getquotes",
        "-a",
        "Author1",
        "-t",
        "#FF0000",
        "-m",
        "25",
        "-l",
        "log.txt",
        "-r",
        "-i",
        "-o",
        "-v",
        "-C",
        "conf.toml",
        "-M",
    ]);
    assert_eq!(args.authors, Some("Author1".to_string()));
    assert_eq!(args.theme_color, Some("#FF0000".to_string()));
    assert_eq!(args.max_tries, Some(25));
    assert_eq!(args.log_file, Some("log.txt".to_string()));
    assert!(args.rainbow_mode);
    assert!(args.init_cache);
    assert!(args.offline);
    assert!(args.version);
    assert_eq!(args.config, Some("conf.toml".to_string()));
    assert!(args.migrate_config);
}

#[test]
fn test_command_factory() {
    let cmd = Args::command();
    assert_eq!(cmd.get_name(), "getquotes");
    // Ensure basic command structure is valid
    cmd.clone().build();
}
