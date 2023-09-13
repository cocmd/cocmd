#!/bin/bash

set -e

echo "Testing cocmd..."
cd cocmd
cargo make test-binary --profile release

echo "Building cocmd and creating AppImage"
cargo make create-app-image --profile release

cd ..
cp cocmd/target/linux/AppImage/out/Cocmd-*.AppImage Cocmd-X11.AppImage
sha256sum Cocmd-X11.AppImage > Cocmd-X11.AppImage.sha256.txt
ls -la

echo "Copying to mounted volume"
cp Cocmd-X11* /shared
