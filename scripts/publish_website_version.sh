#!/bin/bash

set -e

if [[ -z "$VERSION" ]]; then
  echo "Missing target VERSION environment variable, please specify it"
  exit 1
fi

# Removing the v suffix, if present
VERSION=${VERSION#"v"}

echo "Cloning website repository"

rm -rf target/website
mkdir -p target/website
pushd target
git clone git@github.com:cocmd/website.git

pushd website

sed "s/CURRENT_STABLE_VERSION = .*/CURRENT_STABLE_VERSION = 'v$VERSION';/" docusaurus.config.js > docusaurus.config.js.bak
mv docusaurus.config.js.bak docusaurus.config.js

yarn
yarn build && yarn deploy

echo "Committing version update"
git add docusaurus.config.js
git commit -m "Version bump: $VERSION"

echo "Pushing changes"
git push


echo "Done!"