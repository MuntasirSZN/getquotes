
use builtin;
use str;

set edit:completion:arg-completer[getquotes] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'getquotes'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'getquotes'= {
            cand -a 'Specify a list of authors to fetch quotes from'
            cand --authors 'Specify a list of authors to fetch quotes from'
            cand -t 'Set the theme color for the displayed quotes'
            cand --theme-color 'Set the theme color for the displayed quotes'
            cand -m 'Set the maximum number of tries to fetch a quote'
            cand --max-tries 'Set the maximum number of tries to fetch a quote'
            cand -l 'Specify the log file path'
            cand --log-file 'Specify the log file path'
            cand -C 'Use a custom TOML configuration file'
            cand --config 'Use a custom TOML configuration file'
            cand -c 'Generate shell completion script'
            cand --completion 'Generate shell completion script'
            cand -r 'Enable rainbow mode for random quote colors'
            cand --rainbow-mode 'Enable rainbow mode for random quote colors'
            cand -i 'Initialize the quote cache for offline mode'
            cand --init-cache 'Initialize the quote cache for offline mode'
            cand -o 'Run in offline mode, using cached quotes'
            cand --offline 'Run in offline mode, using cached quotes'
            cand -v 'Print version information'
            cand --version 'Print version information'
            cand -M 'Migrate JSON config to TOML format (will be removed in next major release)'
            cand --migrate-config 'Migrate JSON config to TOML format (will be removed in next major release)'
            cand -h 'Print help'
            cand --help 'Print help'
        }
    ]
    $completions[$command]
}
