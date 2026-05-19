module completions {

  def "nu-complete getquotes rainbow_mode" [] {
    [ "true" "false" ]
  }

  def "nu-complete getquotes completion" [] {
    [ "bash" "elvish" "fish" "powershell" "zsh" "nushell" ]
  }

  # A simple cli tool to get quotes in your terminal using WikiQuotes
  export extern getquotes [
    --authors(-a): string     # Specify a list of authors to fetch quotes from
    --theme-color(-t): string # Set the theme color for the displayed quotes
    --max-tries(-m): string   # Set the maximum number of tries to fetch a quote
    --log-file(-l): string    # Specify the log file path
    --rainbow-mode(-r): string@"nu-complete getquotes rainbow_mode" # Enable rainbow mode for gradient rainbow quote colors
    --init-cache(-i)          # Initialize the quote cache for offline mode
    --offline(-o)             # Run in offline mode, using cached quotes
    --version(-v)             # Print version information
    --config(-C): string      # Use a custom TOML configuration file
    --completion(-c): string@"nu-complete getquotes completion" # Generate shell completion script
    --help(-h)                # Print help
  ]

}

export use completions *
