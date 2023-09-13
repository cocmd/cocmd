#!/usr/bin/env bash
set -e
TARGET_DIR=$(pwd)/target/linux/AppImage
BUILD_DIR=$TARGET_DIR/build
OUTPUT_DIR=$TARGET_DIR/out
BASE_DIR=$(pwd)

rm -Rf "$TARGET_DIR"
mkdir -p $OUTPUT_DIR
mkdir -p $BUILD_DIR

echo Building AppImage into $OUTPUT_DIR
pushd $OUTPUT_DIR
$BASE_DIR/scripts/vendor-app-image/linuxdeploy*.AppImage --appimage-extract-and-run -e "$BASE_DIR/$EXEC_PATH" \
  -d "$BASE_DIR/cocmd/src/res/linux/cocmd.desktop" \
  -i "$BASE_DIR/cocmd/src/res/linux/icon.png" \
  --appdir $BUILD_DIR \
  --output appimage
chmod +x ./Cocmd*.AppImage

# Apply a workaround to fix this issue: https://github.com/federico-terzi/cocmd/issues/900
# See: https://github.com/project-slippi/Ishiiruka/issues/323#issuecomment-977415376

echo "Applying patch for libgmodule"

./Cocmd*.AppImage --appimage-extract
rm -Rf ./Cocmd*.AppImage
rm -Rf squashfs-root/usr/lib/libgmodule*
$BASE_DIR/scripts/vendor-app-image/appimagetool*.AppImage --appimage-extract-and-run -v squashfs-root
rm -Rf squashfs-root

popd