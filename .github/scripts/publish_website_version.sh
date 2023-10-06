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

sed "s/RELEASE_TAG=.*/RELEASE_TAG=\"v$VERSION\"/" ./static/linux/install.sh > ./static/linux/install.sh.bak
mv ./static/linux/install.sh.bak ./static/linux/install.sh

cocmd docs > ./docs/cli-docs.md

yarn
yarn build && yarn deploy

echo "Committing version update"
git add docusaurus.config.js
git commit -m "Version bump: $VERSION"

echo "Pushing changes"
git push -f


echo "Done!"