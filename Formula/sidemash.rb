class Sidemash < Formula
  desc "Command Line Interface to use Sidemash Cloud"
  homepage "https://github.com/sidemash/homebrew-sidemash"
  url "https://s3.eu-west-1.amazonaws.com/public.sidemash.io/code/sdk-repo-keys/sidemash-cli-brew/sidemash-mac.tar.gz"
  sha256 "8eb0338627811a39363ae8867dc1fd117da06e32bc59336ba07219240b8ce0cf"
  version "0.1.0"

  def install
    bin.install "sidemash"
  end
end
