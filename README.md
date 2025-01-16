# 📜 GetQuotes - Fetch Inspirational Quotes from the Command Line 🚀

![License](https://img.shields.io/static/v1.svg?style=for-the-badge&label=License&message=MIT&logoColor=d9e0ee&colorA=363a4f&colorB=b7bdf8)
![GitHub issues](https://img.shields.io/github/issues/MuntasirSZN/getquotes?colorA=363a4f&colorB=f5a97f&style=for-the-badge)
![GitHub stars](https://img.shields.io/github/stars/MuntasirSZN/getquotes?style=for-the-badge&logo=andela&color=FFB686&logoColor=D9E0EE&labelColor=292324)
![Last commit](https://img.shields.io/github/last-commit/MuntasirSZN/getquotes?&style=for-the-badge&color=FFB1C8&logoColor=D9E0EE&labelColor=292324)

**GetQuotes** is a powerful command-line tool written in Rust that fetches and displays inspirational quotes directly from Wikiquote. Whether you're looking for motivation, wisdom, or just a quick pick-me-up, GetQuotes has got you covered. 🌟

---

## 📥 Installation

You can install **GetQuotes** using `cargo`, the Rust package manager:

```bash
cargo install getquotes
```

Alternatively, you can clone the repository and build it from source:

```bash
git clone https://github.com/MuntasirSZN/getquotes.git
cd getquotes
cargo build --release
```

---

## 🛠️ Configuration

GetQuotes can be configured using a JSON configuration file. The configuration file should adhere to the schema defined in `config.schema.json`. Here's an example of a configuration file:

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

---

## 🛠️ Usage

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

---

## 🚀 Features

- **Fetch Quotes**: Get random quotes from Wikiquote. 📜
- **Custom Authors**: Specify authors to fetch quotes from. 🖋️
- **Rainbow Mode**: Display quotes in random colors. 🌈
- **Offline Mode**: Use cached quotes when offline. 📴
- **Configurable**: Customize theme color, log file, and more via a JSON configuration file. 🛠️
- **Lightweight**: Fast and efficient, written in Rust. ⚡

---

## 🤝 Contributing

We welcome contributions from the community! If you'd like to contribute, please follow these steps:

1. Fork the repository. 🍴
1. Create a new branch. 🌿
1. Make your changes. ✏️
1. Submit a pull request. 🚀

Please make sure to follow our [Code of Conduct](CODE_OF_CONDUCT.md) and [Contribution Guidelines](CONTRIBUTING.md).

### Configuration Schema

The configuration schema is defined in `config.schema.json`. Any contributions affecting the configuration should update this schema accordingly.

---

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Wikiquote**: For providing the quotes. 📖
- **Rust Community**: For the amazing ecosystem and tools. 🦀
- **Inspiration**: This project was inspired by the need for a simple, daily dose of inspiration. 🌟

---

## 📬 Contact

If you have any questions, feel free to reach out:

- **MuntasirSZN**: [GitHub](https://github.com/MuntasirSZN) | [Email](mailto:muntasir.joypurhat@gmail.com)

---

## 🌟 Show Your Support

If you find this project useful, please give it a ⭐️ on [GitHub](https://github.com/MuntasirSZN/getquotes)!

---

Happy quoting! 🎉
