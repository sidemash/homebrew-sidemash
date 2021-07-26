class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "4b46a4dcdfb74868910e50f07ab7e8eccd38597d3af4119382bf53c7ac2343c3"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
