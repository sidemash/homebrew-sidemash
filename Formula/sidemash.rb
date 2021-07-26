class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "53976731a0888d49e0f329940a28995775fd4927b6f5a04d843f5a6e6623644a"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
