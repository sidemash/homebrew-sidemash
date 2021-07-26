class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "e6a0e559217d309ec4841b6a2dd6ccf0372c3b4b23b430e7a3a2ab531969551c"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
