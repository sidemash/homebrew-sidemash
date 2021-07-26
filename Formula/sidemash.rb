class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "c585f9ab28b17b9ebcbce7fa74a7c680672c965cc0cad7eb2ee4ffb3a66323b2"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
