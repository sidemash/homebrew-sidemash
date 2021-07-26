class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "da53f353817318888bfefc88ce2a6be0730d4399927de4e9adb98a4fe8e9d380"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
