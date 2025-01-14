# getquotes(1)

## NAME

getquotes - Fetch and display quotes from Wikiquote

## SYNOPSIS

getquotes [OPTIONS]

## DESCRIPTION

getquotes is a command-line tool that fetches quotes from Wikiquote and displays them in the terminal. It supports various options to customize the output and behavior.

## OPTIONS

- -h, --help: Display help information.
- --authors <authors>: Specify a list of authors to fetch quotes from.
- --theme-color <color>: Set the theme color for the displayed quotes.
- --max-tries <number>: Set the maximum number of tries to fetch a quote.
- --log-file <file>: Specify the log file path.
- --rainbow-mode: Enable rainbow mode for random quote colors.
- --init-cache: Initialize the quote cache for offline mode.
- --offline: Run in offline mode, using cached quotes.

## EXAMPLES

getquotes
getquotes --authors "Albert Einstein,Mahatma Gandhi"
getquotes --rainbow-mode
getquotes --offline

## EXIT STATUS

- 0: Success.
- 1: General error.
- 2: Configuration error.
- 3: Network error.
- 4: Cache error.

## SEE ALSO

mdbook(1), sqlite3(1)
