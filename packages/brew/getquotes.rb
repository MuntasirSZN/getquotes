class Getquotes < Formula
  desc '💭 GetQuotes is a simple cli tool to get quotes in your terminal using WikiQuotes, Written In Rust🦀'
  homepage 'https://github.com/MuntasirSZN/getquotes'
  license 'MIT'
  version 'v0.3.4'

  on_macos do
    if Hardware::CPU.arm?
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.3.4/getquotes-aarch64-apple-darwin.tar.gz'
      sha256 'f832228f62862671229006edfb445b5ae4bd16264a90abf0ed6422998eed0cde'
    else
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.3.4/getquotes-x86_64-apple-darwin.tar.gz'
      sha256 '88798a2925837f6980733f10e72d5716862ea9c24b0d2191803698c57c01ab69'
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.3.4/getquotes-aarch64-unknown-linux-gnu.tar.gz'
      sha256 '6d4227ecaeee2eadccd2b62c48851ec1c300f0311175b0df40d8bed85d457fc2'
    else
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.3.4/getquotes-x86_64-unknown-linux-gnu.tar.gz'
      sha256 'b16842e6f1cc6ebf091a5122594e115bf46406c85a7ca8318433fe66067b4823'
    end
  end

  def install
    bin.install 'getquotes'
    man1.install 'man/getquotes.1'
  end

  test do
    # Test version output
    assert_match 'getquotes v', shell_output("#{bin}/getquotes --version")

    # Test help output
    assert_match 'Usage: getquotes', shell_output("#{bin}/getquotes --help")

    # Verify man page installation
    assert_predicate man1 / 'getquotes.1', :exist?

    # Basic execution test
    system "#{bin}/getquotes"
  end
end
