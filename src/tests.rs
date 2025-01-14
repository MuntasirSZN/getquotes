mod tests {
    use super::*;
    #[cfg(test)]
    mod config_tests {
        use super::*;
        use tempdir::TempDir;
        use std::env;
        use std::fs::File;
        use std::io::Write;

        #[test]
        fn test_load_existing_config() {
            let temp_dir = TempDir::new("config_test").unwrap();
            let config_path = temp_dir.path().join("config.json");
            let sample_config = r#"
            {
                "authors": ["Author1", "Author2"],
                "theme_color": "#1A2B3C",
                "max_tries": 10,
                "log_file": "custom.log",
                "rainbow_mode": true
            }
            "#;
            File::create(&config_path)
                .unwrap()
                .write_all(sample_config.as_bytes())
                .unwrap();
            env::set_var("HOME", temp_dir.path().to_str().unwrap());

            let cfg = load_or_create_config().unwrap();
            assert_eq!(cfg.authors, vec!["Author1".to_string(), "Author2".to_string()]);
            assert_eq!(cfg.theme_color, "#1A2B3C".to_string());
            assert_eq!(cfg.max_tries, 10);
            assert_eq!(cfg.log_file, "custom.log".to_string());
            assert_eq!(cfg.rainbow_mode, true);
        }

        #[test]
        fn test_create_new_config() {
            let temp_dir = TempDir::new("config_test").unwrap();
            env::set_var("HOME", temp_dir.path().to_str().unwrap());

            let cfg = load_or_create_config().unwrap();
            assert_eq!(cfg.authors, default_authors());
            assert_eq!(cfg.theme_color, "#B7FFFA".to_string());
            assert_eq!(cfg.max_tries, 30);
            assert_eq!(cfg.log_file, "getquotes.log".to_string());
            assert_eq!(cfg.rainbow_mode, false);
        }

        #[test]
        fn test_parse_hex_color() {
            assert_eq!(parse_hex_color("#1A2B3C"), Some((0x1A, 0x2B, 0x3C)));
            assert_eq!(parse_hex_color("1A2B3C"), None);
            assert_eq!(parse_hex_color("#1A2B3"), None);
            assert_eq!(parse_hex_color("#XYZABC"), None);
        }
    }
}
