class Teamtailor < Formula
  desc "Helps out with your development environment"
  homepage "https://github.com/bzf/teamtailor-cli"
  version "0.1.0"

  if OS.mac?
    url "https://github.com/bzf/teamtailor-cli/releases/download/release/teamtailor-macos.zip"
  elsif OS.linux?
    url "https://github.com/bzf/teamtailor-cli/releases/download/release/teamtailor-linux.zip"
  end

  def install
    bin.install "teamtailor"
  end
end
