class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "52659934dbf62f8f87f6ea3fd009fbf1e4d3d8859eeffb8e3e20cde8d7f39642"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
