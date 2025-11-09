#!/bin/bash
set -e

echo "Creating XCFramework for VoxPDF..."

# Ensure iOS builds exist
if [ ! -f "target/aarch64-apple-ios/release/libvoxpdf_core.a" ]; then
    echo "Error: iOS builds not found. Run ./scripts/build-ios.sh first."
    exit 1
fi

# Create XCFramework output directory
rm -rf build/VoxPDFCore.xcframework
mkdir -p build

# Create XCFramework
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/libvoxpdf_core.a \
    -headers ../voxpdf-swift/Sources/VoxPDF/include \
    -library target/aarch64-apple-ios-sim/release/libvoxpdf_core.a \
    -headers ../voxpdf-swift/Sources/VoxPDF/include \
    -library target/x86_64-apple-ios/release/libvoxpdf_core.a \
    -headers ../voxpdf-swift/Sources/VoxPDF/include \
    -output build/VoxPDFCore.xcframework

echo "✅ XCFramework created at build/VoxPDFCore.xcframework"
