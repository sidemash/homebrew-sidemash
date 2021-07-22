class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-cli-mac.tar.gz"
  sha256 "9d8c11c894e255614a203cfa22cb15750c34a107074b9482a16f25f892ae6365"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end