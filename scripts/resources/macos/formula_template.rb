cask "cocmd" do
  version "{{{VERSION}}}"

  if Hardware::CPU.intel?
    url "https://github.com/federico-terzi/cocmd/releases/download/v#{version}/Cocmd-Mac-Intel.zip"
    sha256 "{{{INTEL_SHA}}}"
  else
    url "https://github.com/federico-terzi/cocmd/releases/download/v#{version}/Cocmd-Mac-M1.zip"
    sha256 "{{{M1_SHA}}}"
  end

  name "Cocmd"
  desc "A Privacy-first, Cross-platform Text Expander"
  homepage "https://cocmd.org/"

  app "Cocmd.app"

  zap trash: "~/Library/Caches/cocmd"
end