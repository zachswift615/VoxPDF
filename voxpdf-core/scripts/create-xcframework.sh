#!/bin/bash
set -e

echo "Creating XCFramework for VoxPDF..."

# Ensure iOS builds exist
if [ ! -f "target/aarch64-apple-ios/release/libvoxpdf_core.a" ]; then
    echo "Error: iOS builds not found. Run ./scripts/build-ios.sh first."
    exit 1
fi

# Create build directories
rm -rf build/VoxPDFCore.xcframework
mkdir -p build/simulator

echo "Combining simulator architectures (arm64 + x86_64) into fat library..."
lipo -create \
    target/aarch64-apple-ios-sim/release/libvoxpdf_core.a \
    target/x86_64-apple-ios/release/libvoxpdf_core.a \
    -output build/simulator/libvoxpdf_core.a

echo "Creating XCFramework..."
xcodebuild -create-xcframework \
    -library target/aarch64-apple-ios/release/libvoxpdf_core.a \
    -headers ../voxpdf-swift/Sources/VoxPDF/include \
    -library build/simulator/libvoxpdf_core.a \
    -headers ../voxpdf-swift/Sources/VoxPDF/include \
    -output build/VoxPDFCore.xcframework

# Show size breakdown
echo ""
echo "📦 XCFramework size breakdown:"
echo "  Total:     $(du -sh build/VoxPDFCore.xcframework | cut -f1)"
echo "  Device:    $(du -sh build/VoxPDFCore.xcframework/ios-arm64 | cut -f1)"
echo "  Simulator: $(du -sh build/VoxPDFCore.xcframework/ios-arm64_x86_64-simulator | cut -f1)"
echo ""
echo "✅ XCFramework created at build/VoxPDFCore.xcframework"
