class Getquotes < Formula
  desc 'A tool to fetch quotes'
  homepage 'https://github.com/MuntasirSZN/getquotes'
  version 'v0.2.4'

  on_macos do
    if Hardware::CPU.arm?
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.2.4/getquotes-aarch64-apple-darwin'
      sha256 '8e0f379520ef4d08a6c3d946dae1de120629e3ee30dd0c8fc0a4f4d25ec0fb0f'
    else
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.2.4/getquotes-x86_64-apple-darwin'
      sha256 'ae92f1ee6850494d62634f659e9628adeca4e0e4f542944eb0b5d7a2bc9867c2'
    end
  end

  on_linux do
    if Hardware::CPU.arm?
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.2.4/getquotes-aarch64-linux-android'
      sha256 '159a0f364ab581ad0f34f622514ea356e667c2577d9c21fa7418f0dfad9784db'
    else
      url 'https://github.com/MuntasirSZN/getquotes/releases/download/v0.2.4/getquotes-x86_64-unknown-linux-gnu'
      sha256 '930d7099b41c15cc171278e5fab254d2c6968925256443846a3a4aed3a81e304'
    end
  end

  def install
    if OS.mac?
      if Hardware::CPU.arm?
        bin.install 'getquotes-aarch64-apple-darwin' => 'getquotes'
      else
        bin.install 'getquotes-x86_64-apple-darwin' => 'getquotes'
      end
    elsif OS.linux?
      if Hardware::CPU.arm?
        bin.install 'getquotes-aarch64-linux-android' => 'getquotes'
      else
        bin.install 'getquotes-x86_64-unknown-linux-gnu' => 'getquotes'
      end
    else
      # Handle other OS if necessary
    end
  end

  test do
    system "#{bin}/getquotes", '--version'
  end
end
