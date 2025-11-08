# Testing VoxPDF Swift Bindings

## Prerequisites

1. Build Rust library:
   ```bash
   cd voxpdf-core
   cargo build --release
   ```

2. Ensure library is accessible to Swift:
   - macOS: `target/release/libvoxpdf_core.dylib`
   - Linux: `target/release/libvoxpdf_core.so`

## Running Tests

### Option 1: Xcode
1. Open `voxpdf-swift` in Xcode
2. Add library search path to build settings
3. Run tests (Cmd+U)

### Option 2: Command Line
```bash
cd voxpdf-swift
swift test
```

## Expected Results

All tests should pass:
- testOpenSimplePDF
- testOpenNonexistentPDF
- testExtractText
