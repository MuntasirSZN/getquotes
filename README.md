<h1 align="center">📜 getquotes</h1>
<h4 align="center">Fetch Inspirational Quotes from the Command Line 🚀</h4>

<h4 align="center">
  <img src="https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8&logo=MIT" alt="License">
  <img src="https://img.shields.io/github/issues/MuntasirSZN/getquotes?colorA=363a4f&colorB=f5a97f&style=for-the-badge&logo=github" alt="GitHub issues">
  <img src="https://img.shields.io/github/stars/MuntasirSZN/getquotes?style=for-the-badge&logo=andela&color=FFB686&logoColor=D9E0EE&labelColor=292324" alt="GitHub stars">
  <img src="https://img.shields.io/github/last-commit/MuntasirSZN/getquotes?&style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=292324&logo=git" alt="Last commit">
</h4>

**getquotes** is a powerful command-line tool written in Rust that fetches and displays inspirational quotes directly from Wikiquote. Whether you're looking for motivation, wisdom, or just a quick pick-me-up, GetQuotes has got you covered. 🌟
______________________________________________________________________

## 📖 Table of contents

- [🚀 Features](#-features)
- [📥 Installation](#-installation)
- [🛠️ Configuration](#-configuration)
- [🖱️ Usage](#-usage)
- [🤝 Contributing](#-contributing)
- [📜 License](#-license)
- [🙏 Acknowledgments](#-acknowledgments)
- [📬 Contact](#-contact)
- [🌟 Show Your Support](#-show-your-support)

______________________________________________________________________

## 🚀 Features

- **Fetch Quotes**: Get random quotes from Wikiquote. 📜
- **Custom Authors**: Specify authors to fetch quotes from. 🖋️
- **Rainbow Mode**: Display quotes in random colors. 🌈
- **Offline Mode**: Use cached quotes when offline. 📴
- **Configurable**: Customize theme color, log file, and more via a JSON configuration file. 🛠️
- **Lightweight**: Fast and efficient, written in Rust. ⚡

______________________________________________________________________

## 📥 Installation

| Repository | Command To Install | Version |
| ---------- | ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Crates.io | `cargo install getquotes` | [![Crates.io](https://img.shields.io/crates/v/getquotes?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=292324&logo=rust)](https://crates.io/crates/getquotes) |
| AUR | `yay -S getquotes` | [![AUR](https://img.shields.io/aur/version/getquotes?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=292324&logo=archlinux)](https://aur.archlinux.org/packages/getquotes) |
| AUR (Git) | `yay -S getquotes-git` | [![AUR](https://img.shields.io/aur/version/getquotes-git?style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=292324&logo=git)](https://aur.archlinux.org/packages/getquotes-git) |

> [!Note]
> If you are on Arch Linux, you can install GetQuotes from the AUR using `yay` or `paru`.

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

______________________________________________________________________

## 🛠️ Configuration

getquotes can be configured using a JSON configuration file. The configuration file should adhere to the schema defined in `config.schema.json`. Here's an example of a configuration file:

```json
{
  "authors": ["Albert Einstein", "Isaac Newton"],
  "theme_color": "#FF5733",
  "max_tries": 50,
  "log_file": "custom_getquotes.log"
}
```

### Properties

- **authors**: An array of author names to fetch quotes from. At least one author must be specified.
- **theme_color**: A hex color code (with or without a leading #) for theming the output.
- **max_tries**: The maximum number of attempts to find a quote (between 1 and 100, default is 30).
- **log_file**: The path to the log file (default is "getquotes.log").

______________________________________________________________________

## 🖱️ Usage

### Basic Usage

To fetch and display a random quote, simply run:

```bash
getquotes
```

### Customizing Quotes

You can specify authors to fetch quotes from:

```bash
getquotes --authors "Albert Einstein,Mahatma Gandhi"
```

### Theme Color

Set the theme color using the configuration file or environment variables. Command-line options for theme color are not available.

### Log File

Specify the log file path in the configuration file or use the default "getquotes.log".

### Rainbow Mode 🌈

Enable rainbow mode for a colorful display:

```bash
getquotes --rainbow-mode
```

### Offline Mode

Run in offline mode using cached quotes:

```bash
getquotes --offline
```

### Initialize Cache

Initialize the quote cache for offline mode:

```bash
getquotes --init-cache
```

### Configuration File

Specify the path to the configuration file using the `--config` option:

```bash
getquotes --config /path/to/config.json
```

### Help

Display help information:

```bash
getquotes --help
```

______________________________________________________________________

## 🤝 Contributing

We welcome contributions from the community! If you'd like to contribute, please follow these steps:

1. Fork the repository. 🍴
1. Create a new branch. 🌿
1. Make your changes. ✏️
1. Submit a pull request. 🚀

Please make sure to follow our [Code of Conduct](CODE_OF_CONDUCT.md) and [Contribution Guidelines](CONTRIBUTING.md).

### Configuration Schema

The configuration schema is defined in `config.schema.json`. Any contributions affecting the configuration should update this schema accordingly.

______________________________________________________________________

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

______________________________________________________________________

## 🙏 Acknowledgments

- **Wikiquote**: For providing the quotes. 📖
- **Rust Community**: For the amazing ecosystem and tools. 🦀
- **Inspiration**: This project was inspired by the need for a simple, daily dose of inspiration. 🌟

______________________________________________________________________

## 📬 Contact

If you have any questions, feel free to reach out:

- **MuntasirSZN**: [GitHub](https://github.com/MuntasirSZN) | [Email](mailto:muntasir.joypurhat@gmail.com)

______________________________________________________________________

## 🌟 Show Your Support

If you find this project useful, please give it a ⭐️ on [GitHub](https://github.com/MuntasirSZN/getquotes)!
