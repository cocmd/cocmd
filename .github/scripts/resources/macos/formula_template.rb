class Cocmd < Formula
  version "{{{VERSION}}}"

  if Hardware::CPU.intel?
    url "https://github.com/cocmd/cocmd/releases/download/v#{version}/cocmd-x86_64-apple-darwin.tar.gz"
    sha256 "{{{INTEL_SHA}}}"
  else
    url "https://github.com/cocmd/cocmd/releases/download/v#{version}/cocmd-aarch64-apple-darwin.tar.gz"
    sha256 "{{{M1_SHA}}}"
  end

  desc "Streamlined Command Line Operations for Dev Teams"
  homepage "https://cocmd.org/"

  def install
    bin.install "cocmd"
  end

  zap trash: "~/Library/Caches/cocmd"
end