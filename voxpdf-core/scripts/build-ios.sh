#!/bin/bash
set -e

echo "Building VoxPDF for iOS targets..."

# Set iOS deployment target
export IPHONEOS_DEPLOYMENT_TARGET=15.0

# Build for iOS device (ARM64)
echo "  → Building for iOS device (aarch64-apple-ios)..."
cargo build --release --target aarch64-apple-ios

# Build for iOS simulator (ARM64, M1+)
echo "  → Building for iOS simulator ARM64 (aarch64-apple-ios-sim)..."
cargo build --release --target aarch64-apple-ios-sim

# Build for iOS simulator (x86_64, Intel)
echo "  → Building for iOS simulator x86_64 (x86_64-apple-ios)..."
cargo build --release --target x86_64-apple-ios

echo "✅ iOS builds complete!"
echo ""
echo "Output locations:"
echo "  Device:       target/aarch64-apple-ios/release/libvoxpdf_core.a"
echo "  Simulator M1: target/aarch64-apple-ios-sim/release/libvoxpdf_core.a"
echo "  Simulator Intel: target/x86_64-apple-ios/release/libvoxpdf_core.a"
