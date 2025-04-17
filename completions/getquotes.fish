complete -c getquotes -s a -l authors -d 'Specify a list of authors to fetch quotes from' -r
complete -c getquotes -s t -l theme-color -d 'Set the theme color for the displayed quotes' -r
complete -c getquotes -s m -l max-tries -d 'Set the maximum number of tries to fetch a quote' -r
complete -c getquotes -s l -l log-file -d 'Specify the log file path' -r
complete -c getquotes -s C -l config -d 'Use a custom TOML configuration file' -r
complete -c getquotes -s c -l completion -d 'Generate shell completion script' -r -f -a "bash\t''
elvish\t''
fish\t''
powershell\t''
zsh\t''
nushell\t''"
complete -c getquotes -s r -l rainbow-mode -d 'Enable rainbow mode for random quote colors'
complete -c getquotes -s i -l init-cache -d 'Initialize the quote cache for offline mode'
complete -c getquotes -s o -l offline -d 'Run in offline mode, using cached quotes'
complete -c getquotes -s v -l version -d 'Print version information'
complete -c getquotes -s M -l migrate-config -d 'Migrate JSON config to TOML format (will be removed in next major release)'
complete -c getquotes -s h -l help -d 'Print help'
