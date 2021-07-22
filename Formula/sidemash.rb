class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-cli-mac.tar.gz"
  sha256 "9be31e34e795d8822fdb010a3c63e7167b317cc1463c9b8f0e916fdba6721f83"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end