# VoxPDF Swift

Swift bindings for VoxPDF, a PDF text extraction library optimized for TTS.

## Building

First, build the Rust library:

```bash
cd ../voxpdf-core
cargo build --release
```

Then build the Swift package:

```bash
cd ../voxpdf-swift
swift build
```

## Testing

Set library path and run tests:

```bash
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test
```

## Usage

```swift
import VoxPDF

let url = URL(fileURLWithPath: "document.pdf")
let doc = try PDFDocument(url: url)

print("Pages: \(doc.pageCount)")

let text = try doc.text(page: 0)
print(text)
```

## iOS Build

To use VoxPDF in an iOS project, build the XCFramework:

```bash
# Build Rust library for all iOS targets
cd voxpdf-core
./scripts/build-ios.sh

# Create XCFramework
./scripts/create-xcframework.sh
```

The XCFramework will be created at `voxpdf-core/build/VoxPDFCore.xcframework`.

Add this XCFramework to your Xcode project:
1. Drag `VoxPDFCore.xcframework` into your Xcode project
2. Add VoxPDF Swift package to your project
3. Import and use: `import VoxPDF`

## Supported Platforms

- iOS 15.0+
- macOS 12.0+
- Device: ARM64 (iPhone, iPad)
- Simulator: ARM64 (M1+ Mac) and x86_64 (Intel Mac)
