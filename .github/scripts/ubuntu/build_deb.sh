#!/bin/bash

set -e

echo "Installing cargo-deb"
cargo install cargo-deb --version 1.34.0

cd cocmd

echo "Building X11 deb package"
cargo deb -p cocmd 

# echo "Building Wayland deb package"
# cargo deb -p cocmd --variant wayland 

cd ..
cp cocmd/target/debian/cocmd_*.deb cocmd-debian-x11-amd64.deb
sha256sum cocmd-debian-x11-amd64.deb > cocmd-debian-x11-amd64-sha256.txt
# cp cocmd/target/debian/cocmd-wayland*.deb cocmd-debian-wayland-amd64.deb
# sha256sum cocmd-debian-wayland-amd64.deb > cocmd-debian-wayland-amd64-sha256.txt
ls -la

echo "Copying to mounted volume"
cp cocmd-debian-* /shared
