<h1 align="center">📜 getquotes</h1>
<h4 align="center">Fetch Inspirational Quotes from the Command Line 🚀</h4>

<h4 align="center">
  <img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8&logo=homepage&labelColor=1E1E2E" alt="License">
  <img src="https://img.shields.io/github/issues/MuntasirSZN/getquotes?colorA=363a4f&colorB=f5a97f&style=for-the-badge&logo=github&labelColor=1E1E2E" alt="GitHub issues">
  <img src="https://img.shields.io/github/stars/MuntasirSZN/getquotes?style=for-the-badge&logo=andela&color=FFB686&logoColor=D9E0EE&labelColor=1E1E2E" alt="GitHub stars">
  <img src="https://img.shields.io/github/last-commit/MuntasirSZN/getquotes?&style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=1E1E2E&logo=git" alt="Last commit">
  <img alt="Repo size" src="https://img.shields.io/github/languages/code-size/MuntasirSZN/getquotes?style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA0NDggNTEyIj48IS0tIUZvbnQgQXdlc29tZSBGcmVlIDYuNy4yIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlL2ZyZWUgQ29weXJpZ2h0IDIwMjUgRm9udGljb25zLCBJbmMuLS0%2BPHBhdGggc3Ryb2tlPSIjQ0JBNkY3IiBmaWxsPSIjQ0JBNkY3IiBkPSJNOTYgMEM0MyAwIDAgNDMgMCA5NkwwIDQxNmMwIDUzIDQzIDk2IDk2IDk2bDI4OCAwIDMyIDBjMTcuNyAwIDMyLTE0LjMgMzItMzJzLTE0LjMtMzItMzItMzJsMC02NGMxNy43IDAgMzItMTQuMyAzMi0zMmwwLTMyMGMwLTE3LjctMTQuMy0zMi0zMi0zMkwzODQgMCA5NiAwem0wIDM4NGwyNTYgMCAwIDY0TDk2IDQ0OGMtMTcuNyAwLTMyLTE0LjMtMzItMzJzMTQuMy0zMiAzMi0zMnptMzItMjQwYzAtOC44IDcuMi0xNiAxNi0xNmwxOTIgMGM4LjggMCAxNiA3LjIgMTYgMTZzLTcuMiAxNi0xNiAxNmwtMTkyIDBjLTguOCAwLTE2LTcuMi0xNi0xNnptMTYgNDhsMTkyIDBjOC44IDAgMTYgNy4yIDE2IDE2cy03LjIgMTYtMTYgMTZsLTE5MiAwYy04LjggMC0xNi03LjItMTYtMTZzNy4yLTE2IDE2LTE2eiIvPjwvc3ZnPg%3D%3D&logoColor=CBA6F7&labelColor=1e1e2e&color=B4BEFE">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/MuntasirSZN/getquotes?include_prereleases&sort=semver&display_name=release&style=for-the-badge&logo=data%3Aimage%2Fsvg%2Bxml%3Bbase64%2CPHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48IS0tIUZvbnQgQXdlc29tZSBGcmVlIDYuNy4yIGJ5IEBmb250YXdlc29tZSAtIGh0dHBzOi8vZm9udGF3ZXNvbWUuY29tIExpY2Vuc2UgLSBodHRwczovL2ZvbnRhd2Vzb21lLmNvbS9saWNlbnNlL2ZyZWUgQ29weXJpZ2h0IDIwMjUgRm9udGljb25zLCBJbmMuLS0%2BPHBhdGggZmlsbD0iI0Y1RTBEQyIgc3Ryb2tlPSIjRjVFMERDIiBkPSJNMzQ1IDM5LjFMNDcyLjggMTY4LjRjNTIuNCA1MyA1Mi40IDEzOC4yIDAgMTkxLjJMMzYwLjggNDcyLjljLTkuMyA5LjQtMjQuNSA5LjUtMzMuOSAuMnMtOS41LTI0LjUtLjItMzMuOUw0MzguNiAzMjUuOWMzMy45LTM0LjMgMzMuOS04OS40IDAtMTIzLjdMMzEwLjkgNzIuOWMtOS4zLTkuNC05LjItMjQuNiAuMi0zMy45czI0LjYtOS4yIDMzLjkgLjJ6TTAgMjI5LjVMMCA4MEMwIDUzLjUgMjEuNSAzMiA0OCAzMmwxNDkuNSAwYzE3IDAgMzMuMyA2LjcgNDUuMyAxOC43bDE2OCAxNjhjMjUgMjUgMjUgNjUuNSAwIDkwLjVMMjc3LjMgNDQyLjdjLTI1IDI1LTY1LjUgMjUtOTAuNSAwbC0xNjgtMTY4QzYuNyAyNjIuNyAwIDI0Ni41IDAgMjI5LjV6TTE0NCAxNDRhMzIgMzIgMCAxIDAgLTY0IDAgMzIgMzIgMCAxIDAgNjQgMHoiLz48L3N2Zz4%3D&labelColor=1E1E2E&color=45475A">
</h4>

<p align="center">
  <strong>getquotes</strong> is a powerful command-line tool written in Rust that fetches and displays inspirational quotes directly from Wikiquote. Whether you're looking for motivation, wisdom, or just a quick pick-me-up, GetQuotes has got you covered. 🌟
</p>

## 📖 Table of contents

- [🚀 Features](#-features)
- [📥 Installation](#-installation)
- [🧭 Configuration](#-configuration)
- [💡 Usage](#-usage)
- [🤝 Contributing](#-contributing)
- [📜 License](#-license)
- [🙏 Acknowledgments](#-acknowledgments)
- [🌟 Show Your Support](#-show-your-support)

---

## 🚀 Features

- **Fetch Quotes**: Get random quotes from Wikiquote. 📜
- **Custom Authors**: Specify authors to fetch quotes from. 🖋️
- **Rainbow Mode**: Display quotes with a real rainbow gradient. 🌈
- **Offline Mode**: Use cached quotes when offline. 📴
- **Configurable**: Customize colors, gradients, text styles, layout, log file, and more via a TOML configuration file. 🛠️
- **Lightweight**: Fast and efficient, written in Rust. ⚡

---

## 📥 Installation

| Repository | Command To Install | Version |
| ---------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Crates.io | `cargo install getquotes` | [![Crates.io](https://img.shields.io/crates/v/getquotes?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=1E1E2E&logo=rust)](https://crates.io/crates/getquotes) |
| AUR | `yay -S getquotes` | [![AUR](https://img.shields.io/aur/version/getquotes?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=1E1E2E&logo=archlinux)](https://aur.archlinux.org/packages/getquotes) |
| AUR (Git) | `yay -S getquotes-git` | [![AUR](https://img.shields.io/aur/version/getquotes-git?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=1E1E2E&logo=git)](https://aur.archlinux.org/packages/getquotes-git) |
| Homebrew | `brew install MuntasirSZN/programs/getquotes` | [![Homebrew](https://img.shields.io/badge/dynamic/json.svg?url=https://raw.githubusercontent.com/MuntasirSZN/homebrew-programs/master/Info/g/getquotes.json&query=$.versions.stable&label=Homebrew&labelColor=1E1E2E&style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&logo=homebrew)](https://github.com/MuntasirSZN/homebrew-programs)

> [!Note]
> You can install getquotes from the AUR using `yay` or `paru`, or any other AUR helper.

### Manual Installation

You can clone the repository and build it from source:

```bash
git clone https://github.com/MuntasirSZN/getquotes.git
cd getquotes
cargo build --release
```

Then, copy the binary to your `PATH`:

```bash
cp target/release/getquotes /usr/local/bin
```

Manpages are included in the repository and can be installed using:

```bash
sudo cp man/getquotes.1 /usr/share/man/man1
sudo mandb # To update the manpage database
```

If you are on windows, you can use the ps1xml file, which is a like a manpage for windows powershell. Use the `Get-Help` command to view the manpage.

```ps1
copy .\man\getquotes.ps1xml $env:PSModulePath\getquotes.ps1xml
Get-Help getquotes
```

> [!Important]
> If you are building for android from source, you need the Android NDK installed and the `ANDROID_NDK_HOME` environment variable set to the NDK path.

---

## 🧭 Configuration

getquotes can be configured using a TOML configuration file. The configuration file schema is generated from Rust types with `schemars` and written to `config/config.schema.json`. You can use this schema in editors powered by [Taplo](https://taplo.tamasfe.dev/) or [Tombi](https://tombi-toml.github.io/tombi/). Add this at the top of your config file:

```toml
#:schema https://raw.githubusercontent.com/MuntasirSZN/getquotes/refs/heads/main/config/config.schema.json
```

Here's an example of a configuration file:

```toml
# List of authors to fetch quotes from
authors = [
    "Albert Einstein",
    "Isaac Newton"
]

# Theme color supports hex, rgb/rgba, hsl, or CSS-inspired gradients
theme_color = "conic-gradient(from 90deg, #FF5733, hsl(45, 100%, 50%), rgb(0, 170, 255))"

# Extra styling for the quote text
quote_style = "bold"

# Styling for the author line
author_style = "italic,green"

# Optional styling for nested quotes inside the quote text
nested_quote_style = "underline"

# Maximum number of attempts to fetch a quote
max_tries = 50

# Log file path
log_file = "custom_getquotes.log"

# Enable rainbow mode for a true rainbow gradient
rainbow_mode = false

# Output layout: "default" or "box"
layout = "default"

# Box corner style when layout = "box": "pointy" or "rounded"
box_corners = "pointy"
```

### Properties

- **authors**: An array of author names to fetch quotes from. At least one author must be specified.
- **theme_color**: The base quote color. Supports hex, `rgb(...)`, `rgba(...)`, `hsl(...)`, and CSS-inspired gradient functions such as `linear-gradient(...)`, `radial-gradient(...)`, `conic-gradient(...)`, and repeating variants.
- **quote_style**: Comma-separated quote styling tokens such as `bold`, `italic`, `underline`, `strikethrough`, colors, or gradients.
- **author_style**: Comma-separated styling tokens for the author line.
- **nested_quote_style**: Optional styling applied to quotes found inside the main quote text.
- **max_tries**: The maximum number of attempts to find a quote (between 1 and 100, default is 30).
- **log_file**: The path to the log file (default is "getquotes.log").
- **rainbow_mode**: Overrides the quote color with a rainbow gradient.
- **layout**: `default` keeps the current layout, while `box` renders the quote and author inside a box.
- **box_corners**: Chooses `pointy` (`+`, `|`, `-`) or `rounded` (`╭`, `╮`, `╰`, `╯`, `│`, `─`) corners for box layout.

---

## 💡 Usage

```
A simple cli tool to get quotes in your terminal using WikiQuotes

Usage: getquotes [OPTIONS]

Options:
  -a, --authors <AUTHORS>          Specify a list of authors to fetch quotes from
  -t, --theme-color <THEME_COLOR>  Set the theme color for the displayed quotes
  -m, --max-tries <MAX_TRIES>      Set the maximum number of tries to fetch a quote
  -l, --log-file <LOG_FILE>        Specify the log file path
  -r, --rainbow-mode [<RAINBOW_MODE>]
                                  Enable rainbow mode for gradient rainbow quote colors
  -i, --init-cache                 Initialize the quote cache for offline mode
  -o, --offline                    Run in offline mode, using cached quotes
  -v, --version                    Print version information
  -C, --config <CONFIG>            Use a custom TOML configuration file
  -c, --completion <COMPLETION>    Generate shell completion script [possible values: bash, elvish, fish, powershell, zsh, nushell]
  -h, --help                       Print help

MuntasirSZN <muntasir.joypurhat@gmail.com>
```
---

## 🤝 Contributing

We welcome contributions from the community! If you'd like to contribute, please follow these steps:

1. Fork the repository. 🍴
1. Create a new branch. 🌿
1. Make your changes. ✏️
1. Submit a pull request. 🚀

Please make sure to follow our [Code of Conduct](CODE_OF_CONDUCT.md) and [Contribution Guidelines](CONTRIBUTING.md).

### Configuration Schema

The configuration schema file is `config/config.schema.json`, generated from `Config` via `schemars`.
If you change configuration fields, run:

```bash
cargo run --bin generate_config_schema
```

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Wikiquote**: For providing the quotes. 📖
- **Rust Community**: For the amazing ecosystem and tools. 🦀
- **Inspiration**: This project was inspired by the need for a simple, daily dose of inspiration. 🌟

---

## 🌟 Show Your Support

If you find this project useful, please give it a ⭐️ on [GitHub](https://github.com/MuntasirSZN/getquotes)!
