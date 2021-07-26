class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "88c0cd6f39630a58f9ddd16267a41d5a39eb5f1566e1d92e31dade97b8a9c528"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
