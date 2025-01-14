# 📜 GetQuotes - Fetch Inspirational Quotes from the Command Line 🚀

![GitHub](https://img.shields.io/badge/License-MIT-blue.svg)
![GitHub issues](https://img.shields.io/github/issues/MuntasirSZN/getquotes)
![GitHub stars](https://img.shields.io/github/stars/MuntasirSZN/getquotes)
![GitHub forks](https://img.shields.io/github/forks/MuntasirSZN/getquotes)

**GetQuotes** is a powerful command-line tool written in Rust that fetches and displays inspirational quotes directly from Wikiquote. Whether you're looking for motivation, wisdom, or just a quick pick-me-up, GetQuotes has got you covered. 🌟

______________________________________________________________________

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

______________________________________________________________________

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

### Help

Display help information:

```bash
getquotes --help
```

______________________________________________________________________

## 📂 Project Structure

```
getquotes/
├── Cargo.lock
├── Cargo.toml
├── CHANGELOG.md
├── config.schema.json
├── man/
│   └── getquote.md
├── output.txt
├── quotes.db
├── README.md
├── src/
│   ├── background.rs
│   ├── cache.rs
│   ├── config.rs
│   ├── lib.rs
│   ├── logger.rs
│   ├── main.rs
│   ├── quotes.rs
│   ├── tests.rs
│   └── types.rs
└── tests/
    ├── cache_tests.rs
    ├── common/
    │   └── mod.rs
    ├── config_tests.rs
    ├── integration_tests.rs
    ├── logger_tests.rs
    └── quotes_tests.rs
```

______________________________________________________________________

## 🚀 Features

- **Fetch Quotes**: Get random quotes from Wikiquote. 📜
- **Custom Authors**: Specify authors to fetch quotes from. 🖋️
- **Rainbow Mode**: Display quotes in random colors. 🌈
- **Offline Mode**: Use cached quotes when offline. 📴
- **Lightweight**: Fast and efficient, written in Rust. ⚡

______________________________________________________________________

## 🤝 Contributing

We welcome contributions from the community! If you'd like to contribute, please follow these steps:

1. Fork the repository. 🍴
1. Create a new branch. 🌿
1. Make your changes. ✏️
1. Submit a pull request. 🚀

Please make sure to follow our [Code of Conduct](CODE_OF_CONDUCT.md) and [Contribution Guidelines](CONTRIBUTING.md).

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

______________________________________________________________________

Happy quoting! 🎉
