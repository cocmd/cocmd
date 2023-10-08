cask "cocmd" do
  version "{{{VERSION}}}"

  if Hardware::CPU.intel?
    url "https://github.com/cocmd/cocmd/releases/download/v#{version}/Cocmd-Mac-Intel.zip"
    sha256 "{{{INTEL_SHA}}}"
  else
    url "https://github.com/cocmd/cocmd/releases/download/v#{version}/Cocmd-Mac-M1.zip"
    sha256 "{{{M1_SHA}}}"
  end

  name "Cocmd"
  desc "Streamlined Command Line Operations for Dev Teams"
  homepage "https://cocmd.org/"

  app "Cocmd.app"

  zap trash: "~/Library/Caches/cocmd"
end