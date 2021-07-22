class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-cli-mac.tar.gz"
  sha256 "e652d7bf37c1c54017283109f0f43dfa9c972989648cd729c5dcfeecb5b22449"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end