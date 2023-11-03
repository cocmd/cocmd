#!/bin/bash

set -e

if [[ -z "$VERSION" ]]; then
  echo "Missing target VERSION environment variable, please specify it"
  exit 1
fi

# Removing the v suffix, if present
VERSION=${VERSION#"v"}

rm -Rf target/homebrew
mkdir -p target/homebrew/artifacts

echo "Targeting version $VERSION"
echo "Downloading macOS artifacts"

gh release download v$VERSION --pattern "*apple*" --dir target/homebrew/artifacts

echo "Reading artifacts hashes"
INTEL_SHA=$(cat target/homebrew/artifacts/cocmd-x86_64-apple-darwin.sha256 | awk -F ' ' '{print $1}')
M1_SHA=$(cat target/homebrew/artifacts/cocmd-aarch64-apple-darwin.sha256 | awk -F ' ' '{print $1}')

echo "Cloning tap repository"

pushd target/homebrew
git clone git@github.com:cocmd/homebrew-tap.git

pushd homebrew-tap
echo "Rendering formula template"

cat ../../../.github/scripts/resources/macos/formula_template.rb | sed "s/{{{VERSION}}}/$VERSION/g" | \
  sed "s/{{{INTEL_SHA}}}/$INTEL_SHA/g" | sed "s/{{{M1_SHA}}}/$M1_SHA/g" > ./Formula/cocmd.rb

echo "Committing version update"
git add Formula/cocmd.rb
git commit -m "Version bump: $VERSION"

echo "Pushing changes"
git push

echo "Done!"