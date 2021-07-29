class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "562fdbec406f6f19fe8cd8f4152ba8789b8d4599aa9d2be0d3803b30b5085de6"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
