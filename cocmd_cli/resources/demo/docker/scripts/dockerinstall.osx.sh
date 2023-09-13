# Install Homebrew if not installed
curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh

# Tap the Docker formulae
brew tap docker/cli

# Install Docker
brew install docker

# Start Docker
open /Applications/Docker.app

# Check is installation successful
docker --version