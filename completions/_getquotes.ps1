
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'getquotes' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'getquotes'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'getquotes' {
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Specify a list of authors to fetch quotes from')
            [CompletionResult]::new('--authors', '--authors', [CompletionResultType]::ParameterName, 'Specify a list of authors to fetch quotes from')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Set the theme color for the displayed quotes')
            [CompletionResult]::new('--theme-color', '--theme-color', [CompletionResultType]::ParameterName, 'Set the theme color for the displayed quotes')
            [CompletionResult]::new('-m', '-m', [CompletionResultType]::ParameterName, 'Set the maximum number of tries to fetch a quote')
            [CompletionResult]::new('--max-tries', '--max-tries', [CompletionResultType]::ParameterName, 'Set the maximum number of tries to fetch a quote')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Specify the log file path')
            [CompletionResult]::new('--log-file', '--log-file', [CompletionResultType]::ParameterName, 'Specify the log file path')
            [CompletionResult]::new('-C', '-C ', [CompletionResultType]::ParameterName, 'Use a custom TOML configuration file')
            [CompletionResult]::new('--config', '--config', [CompletionResultType]::ParameterName, 'Use a custom TOML configuration file')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Generate shell completion script')
            [CompletionResult]::new('--completion', '--completion', [CompletionResultType]::ParameterName, 'Generate shell completion script')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Enable rainbow mode for random quote colors')
            [CompletionResult]::new('--rainbow-mode', '--rainbow-mode', [CompletionResultType]::ParameterName, 'Enable rainbow mode for random quote colors')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Initialize the quote cache for offline mode')
            [CompletionResult]::new('--init-cache', '--init-cache', [CompletionResultType]::ParameterName, 'Initialize the quote cache for offline mode')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Run in offline mode, using cached quotes')
            [CompletionResult]::new('--offline', '--offline', [CompletionResultType]::ParameterName, 'Run in offline mode, using cached quotes')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version information')
            [CompletionResult]::new('-M', '-M ', [CompletionResultType]::ParameterName, 'Migrate JSON config to TOML format (will be removed in next major release)')
            [CompletionResult]::new('--migrate-config', '--migrate-config', [CompletionResultType]::ParameterName, 'Migrate JSON config to TOML format (will be removed in next major release)')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
