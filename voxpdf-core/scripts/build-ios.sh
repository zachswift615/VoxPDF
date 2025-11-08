#!/bin/bash
set -e

# Build for iOS targets
echo "Building for iOS devices (aarch64-apple-ios)..."
cargo build --release --target aarch64-apple-ios

echo "Building for iOS simulator (aarch64-apple-ios-sim)..."
cargo build --release --target aarch64-apple-ios-sim

echo "Building for iOS simulator x86_64 (x86_64-apple-ios)..."
cargo build --release --target x86_64-apple-ios

echo "Build complete!"
echo "Device:    target/aarch64-apple-ios/release/libvoxpdf_core.a"
echo "Simulator: target/aarch64-apple-ios-sim/release/libvoxpdf_core.a"
echo "Simulator: target/x86_64-apple-ios/release/libvoxpdf_core.a"
