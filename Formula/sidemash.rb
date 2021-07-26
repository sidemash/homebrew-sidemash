class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "d64642bb36f60d56af1511bf63e1ca77b845a1984b09a48e10aa3116b2aea620"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
