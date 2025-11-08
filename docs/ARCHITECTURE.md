# VoxPDF Architecture

**Cross-platform PDF text extraction library optimized for TTS**

**Core**: Rust
**Bindings**: Swift (iOS/macOS), Kotlin (Android), WASM (Web)
**Inspiration**: Foxit SDK, PSPDFKit (C++ core + platform bindings)

---

## Design Philosophy

### 1. Separation of Concerns

```
┌─────────────────────────────────────┐
│     Platform Layer (Swift/Kotlin)   │  ← Thin wrappers, idiomatic APIs
├─────────────────────────────────────┤
│         C FFI Boundary              │  ← Stable ABI, versioned
├─────────────────────────────────────┤
│      VoxPDF Core (Rust)             │  ← All logic, cross-platform
└─────────────────────────────────────┘
```

**Core Principle**: Platform layers are *thin*. All intelligence lives in Rust.

### 2. Performance First

- Rust/C-level performance for heavy PDF processing
- Zero-copy where possible
- Lazy evaluation and streaming
- Memory-mapped file access
- SIMD optimizations for hot paths (future)

### 3. Safety Without Compromise

- Memory safety from Rust
- No segfaults, no undefined behavior
- Thread-safe by design
- Panic = controlled failure, never UB

### 4. API Stability

- Semantic versioning strictly followed
- FFI ABI is stable between minor versions
- Platform bindings can update independently
- Deprecation policy: 3 releases before removal

---

## Architecture Layers

### Layer 1: VoxPDF Core (Rust)

**Location**: `voxpdf-core/`

**Responsibilities**:
- PDF parsing and structure understanding
- Text extraction algorithms
- Layout analysis (columns, reading order)
- Structure detection (headings, TOC, chapters)
- Word position tracking
- All business logic

**Key Modules**:

```rust
voxpdf-core/
├── src/
│   ├── lib.rs              // Public Rust API
│   ├── ffi.rs              // C FFI exports
│   │
│   ├── pdf/
│   │   ├── parser.rs       // PDF document loading
│   │   ├── page.rs         // Page access
│   │   └── metadata.rs     // Document properties
│   │
│   ├── extraction/
│   │   ├── text.rs         // Text extraction
│   │   ├── words.rs        // Word boundaries & positions
│   │   ├── paragraphs.rs   // Paragraph detection
│   │   └── hyphenation.rs  // Hyphen handling
│   │
│   ├── layout/
│   │   ├── columns.rs      // Column detection
│   │   ├── reading_order.rs // Text flow analysis
│   │   ├── regions.rs      // Text block clustering
│   │   └── geometry.rs     // Bounding box math
│   │
│   ├── structure/
│   │   ├── toc.rs          // Table of contents
│   │   ├── chapters.rs     // Chapter detection
│   │   ├── headings.rs     // Heading analysis
│   │   └── outline.rs      // PDF outline metadata
│   │
│   └── models/
│       ├── document.rs     // Core data structures
│       ├── paragraph.rs
│       ├── word.rs
│       └── chapter.rs
│
└── Cargo.toml
```

**Dependencies**:
```toml
[dependencies]
mupdf-sys = "0.1"          # MuPDF bindings (PDF parsing)
# OR
lopdf = "0.30"             # Pure Rust alternative

unicode-segmentation = "1.10"  # Word/grapheme boundaries
once_cell = "1.19"         # Lazy statics
thiserror = "1.0"          # Error types
serde = { version = "1.0", features = ["derive"] }  # Serialization
```

**Public Rust API** (for Rust users):
```rust
pub struct PDFDocument {
    // ...
}

impl PDFDocument {
    pub fn open(path: impl AsRef<Path>) -> Result<Self>;
    pub fn extract(&self) -> Result<DocumentContent>;
    pub fn page(&self, index: usize) -> Option<&Page>;
}

pub struct DocumentContent {
    pub paragraphs: Vec<Paragraph>,
    pub chapters: Vec<Chapter>,
    pub metadata: Metadata,
}
```

### Layer 2: C FFI Boundary

**Location**: `voxpdf-core/src/ffi.rs`

**Responsibilities**:
- Expose Rust functionality via C ABI
- Handle null pointers, error codes
- Manage memory across FFI boundary
- Type conversion (Rust ↔ C)

**Design Principles**:
- No panics across FFI (catch and return error codes)
- Clear ownership (who frees what)
- Opaque pointers for complex types
- Simple C types for primitives

**Example FFI**:
```rust
// ffi.rs
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

#[repr(C)]
pub struct CVoxPDFDocument {
    _private: [u8; 0],  // Opaque
}

#[repr(C)]
pub struct CWordPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub page: u32,
}

#[no_mangle]
pub extern "C" fn voxpdf_open(
    path: *const c_char,
    error_out: *mut c_int,
) -> *mut CVoxPDFDocument {
    // Convert path, handle errors, return opaque pointer
}

#[no_mangle]
pub extern "C" fn voxpdf_extract(
    doc: *const CVoxPDFDocument,
    error_out: *mut c_int,
) -> *mut CVoxPDFContent {
    // ...
}

#[no_mangle]
pub extern "C" fn voxpdf_free_document(doc: *mut CVoxPDFDocument) {
    // ...
}
```

**Memory Management**:
- Rust owns all memory
- FFI returns opaque pointers
- Platform bindings call `voxpdf_free_*` when done
- Clear documentation of ownership

**Error Handling**:
```rust
#[repr(C)]
pub enum CVoxPDFError {
    Ok = 0,
    InvalidPath = 1,
    FileNotFound = 2,
    CorruptedPDF = 3,
    OutOfMemory = 4,
    // ...
}
```

### Layer 3: Platform Bindings

#### 3a. Swift Bindings (iOS/macOS)

**Location**: `voxpdf-swift/`

**Responsibilities**:
- Wrap FFI with idiomatic Swift API
- Automatic memory management (deinit calls FFI free)
- Swift concurrency (async/await)
- Error translation (C error codes → Swift Error)

**Structure**:
```
voxpdf-swift/
├── Sources/
│   └── VoxPDF/
│       ├── VoxPDF.swift        // Main API
│       ├── PDFDocument.swift   // Swift document type
│       ├── Models.swift        // Swift data models
│       ├── FFI.swift           // C imports
│       └── Internal/
│           └── FFIBridge.swift // Low-level FFI calls
│
├── Tests/
│   └── VoxPDFTests/
│
└── Package.swift
```

**Swift API**:
```swift
public class PDFDocument {
    private let handle: OpaquePointer

    public init(url: URL) throws {
        var error: Int32 = 0
        guard let handle = voxpdf_open(url.path, &error) else {
            throw VoxPDFError(code: error)
        }
        self.handle = handle
    }

    deinit {
        voxpdf_free_document(handle)
    }

    public func extract() async throws -> DocumentContent {
        // Call FFI, convert types
    }
}

public struct DocumentContent {
    public let paragraphs: [Paragraph]
    public let chapters: [Chapter]
    public let metadata: Metadata
}

public struct Paragraph {
    public let id: UUID
    public let text: String
    public let pageNumber: Int
    public let words: [Word]
}
```

**Package.swift**:
```swift
// Package.swift
let package = Package(
    name: "VoxPDF",
    platforms: [.iOS(.v15), .macOS(.v12)],
    products: [
        .library(name: "VoxPDF", targets: ["VoxPDF"]),
    ],
    targets: [
        .systemLibrary(
            name: "CVoxPDF",
            path: "Sources/CVoxPDF",
            pkgConfig: "voxpdf"
        ),
        .target(
            name: "VoxPDF",
            dependencies: ["CVoxPDF"]
        ),
        .testTarget(
            name: "VoxPDFTests",
            dependencies: ["VoxPDF"],
            resources: [.copy("TestPDFs")]
        ),
    ]
)
```

#### 3b. Kotlin Bindings (Android) - Future

**Location**: `voxpdf-kotlin/`

**Similar structure**:
- JNI wrapper around FFI
- Gradle-based build
- Idiomatic Kotlin API
- Coroutines support

#### 3c. WASM Bindings (Web) - Future

**Location**: `voxpdf-wasm/`

- `wasm-bindgen` for JS interop
- TypeScript definitions
- npm package

---

## Data Flow

### Example: Extracting Text

```
User Code (Swift)
    ↓
let doc = try PDFDocument(url: fileURL)
let content = try await doc.extract()
    ↓
[Swift Binding Layer]
    ↓
voxpdf_extract(handle, &error)
    ↓
[FFI Boundary - C ABI]
    ↓
[Rust Core]
    ↓
1. Open PDF with MuPDF/lopdf
2. Extract raw text from each page
3. Detect word boundaries
4. Cluster words into lines
5. Join lines into paragraphs
6. Analyze layout for columns
7. Determine reading order
8. Detect headings via font size
9. Build TOC structure
10. Return DocumentContent
    ↓
[FFI Boundary]
Convert Rust → C structs
    ↓
[Swift Binding Layer]
Convert C structs → Swift types
    ↓
User Code (Swift)
for paragraph in content.paragraphs {
    print(paragraph.text)
}
```

---

## Build Process

### Building Rust Core

```bash
cd voxpdf-core
cargo build --release

# Produces:
# target/release/libvoxpdf_core.dylib (macOS)
# target/release/libvoxpdf_core.so (Linux)
# target/release/voxpdf_core.dll (Windows)
```

### Building for iOS

```bash
# Install targets
rustup target add aarch64-apple-ios
rustup target add x86_64-apple-ios  # Simulator

# Build for iOS
cd voxpdf-core
cargo build --release --target aarch64-apple-ios

# XCFramework (packages all architectures)
./scripts/build-xcframework.sh
```

### Swift Package Consumption

```swift
// Package.swift
dependencies: [
    .package(url: "https://github.com/user/voxpdf-swift", from: "0.1.0")
]
```

Swift Package Manager will:
1. Download pre-built Rust binary (XCFramework)
2. Link against it
3. User gets native Swift API

---

## Cross-Platform Guarantees

### What's Guaranteed Across Platforms

1. **Identical extraction logic**
   - Same algorithms run on iOS/Android/Web
   - Same output for same PDF

2. **API parity**
   - Swift, Kotlin, JS APIs are equivalent
   - Same feature set

3. **Performance characteristics**
   - Extraction speed within 10% across platforms
   - Memory usage similar

### What's Platform-Specific

1. **File I/O**
   - Each platform handles file access differently
   - FFI receives file path as string

2. **UI Integration**
   - Word highlighting is platform-specific
   - Each binding provides platform helpers

3. **Threading model**
   - Swift: async/await
   - Kotlin: Coroutines
   - JS: Promises

---

## Performance Design

### Goals (per-page averages)

| Metric | Target | Stretch |
|--------|--------|---------|
| Extraction time | <100ms | <50ms |
| Memory overhead | <10MB | <5MB |
| Memory per page | <100KB | <50KB |

### Optimization Strategies

**Phase 1 (Correctness)**:
- Focus on getting algorithms right
- Accept naive implementations
- Comprehensive tests

**Phase 2 (Performance)**:
- Profile with real PDFs
- Optimize hot paths
- Consider SIMD for text processing
- Memory pooling for allocations

**Phase 3 (Scale)**:
- Streaming for large PDFs
- Incremental extraction
- Multi-threaded page processing

---

## Testing Strategy

### Rust Core Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_hyphenation_reassembly() {
        let input = vec!["inter-", "rupt"];
        let output = join_hyphenated_words(input);
        assert_eq!(output, vec!["interrupt"]);
    }

    #[test]
    fn test_column_detection() {
        let page = create_test_page_with_columns();
        let layout = detect_layout(&page);
        assert_eq!(layout, ColumnLayout::TwoColumn);
    }
}
```

### FFI Tests

```rust
#[test]
fn test_ffi_document_lifecycle() {
    let path = CString::new("test.pdf").unwrap();
    let mut error = 0;

    let doc = voxpdf_open(path.as_ptr(), &mut error);
    assert!(!doc.is_null());
    assert_eq!(error, 0);

    voxpdf_free_document(doc);
}
```

### Swift Integration Tests

```swift
class VoxPDFTests: XCTestCase {
    func testBasicExtraction() async throws {
        let url = Bundle.module.url(
            forResource: "sample",
            withExtension: "pdf"
        )!

        let doc = try PDFDocument(url: url)
        let content = try await doc.extract()

        XCTAssertGreaterThan(content.paragraphs.count, 0)
        XCTAssertFalse(content.paragraphs[0].text.isEmpty)
    }
}
```

### End-to-End Tests

- Real PDF test suite (100+ PDFs)
- Different generators: LaTeX, Word, InDesign
- Different layouts: 1-col, 2-col, 3-col, mixed
- Edge cases: malformed, encrypted, scanned

---

## Error Handling

### Rust Core

```rust
#[derive(Debug, thiserror::Error)]
pub enum VoxPDFError {
    #[error("Invalid PDF file: {0}")]
    InvalidPDF(String),

    #[error("File not found: {0}")]
    FileNotFound(PathBuf),

    #[error("Unsupported PDF version: {0}")]
    UnsupportedVersion(String),
}

pub type Result<T> = std::result::Result<T, VoxPDFError>;
```

### FFI Boundary

```rust
impl From<VoxPDFError> for CVoxPDFError {
    fn from(err: VoxPDFError) -> Self {
        match err {
            VoxPDFError::InvalidPDF(_) => CVoxPDFError::CorruptedPDF,
            VoxPDFError::FileNotFound(_) => CVoxPDFError::FileNotFound,
            // ...
        }
    }
}
```

### Swift Binding

```swift
public enum VoxPDFError: Error {
    case invalidPDF
    case fileNotFound
    case corruptedPDF
    case unsupportedVersion
    case outOfMemory

    init(code: Int32) {
        // Map C error codes to Swift enum
    }
}
```

---

## Security Considerations

### Input Validation

- Validate PDF structure before processing
- Reject obviously malformed files early
- Limit recursion depth (prevent stack overflow)
- Memory limits (prevent OOM attacks)

### Memory Safety

- Rust prevents most memory bugs by design
- FFI boundary validated (null checks, bounds)
- No unsafe code without careful review

### Fuzzing

```bash
# AFL fuzzing for Rust core
cargo fuzz run pdf_parser
```

---

## Future Architecture Considerations

### Plugin System (v2.0+)

Allow custom extractors:

```rust
pub trait Extractor {
    fn extract(&self, page: &Page) -> Result<Vec<TextBlock>>;
}

// Users can register custom extractors
doc.register_extractor(Box::new(CustomTableExtractor));
```

### GPU Acceleration (v3.0+)

- SIMD for text processing
- GPU for layout analysis (if beneficial)
- Investigate WebGPU for web

---

## Decision Log

### Why Rust over C++?

- Memory safety without runtime overhead
- Better tooling (cargo, clippy, rustfmt)
- Modern language features
- Growing ecosystem
- Easier to attract contributors

### Why MuPDF/lopdf over writing PDF parser?

- PDF spec is 1000+ pages
- Mature libraries exist
- Focus on TTS-specific logic, not parsing
- Can swap backends if needed

### Why FFI instead of native bindings?

- FFI is stable, language-agnostic
- Future languages can bind easily
- Clear separation of concerns
- Matches commercial SDK pattern

---

**Last Updated**: November 7, 2025
**Architecture Version**: 1.0
**Status**: Initial Design
