# VoxPDF FFI + Swift Bindings Completion Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Complete the FFI layer for paragraphs and TOC extraction, then implement full Swift bindings for iOS integration, enabling the full VoxPDF extraction pipeline from Swift.

**Architecture:** Complete the three-layer architecture: Rust core (already done) → C FFI boundary (needs paragraphs + TOC) → Swift bindings (needs full implementation). FFI uses opaque pointers and error codes. Swift provides idiomatic wrappers with automatic memory management.

**Tech Stack:** Rust (mupdf, thiserror), C FFI (repr(C), unsafe), Swift 5.9+, iOS 15+, XCTest

**Current State:**
- ✅ Rust core: 95% complete (all extraction working)
- ⚠️  FFI: 60% complete (text + words done, missing paragraphs + TOC)
- ❌ Swift: 0% complete (voxpdf-swift directory doesn't exist)

---

## Prerequisites

**Working Directory:** `/Users/zachswift/projects/VoxPDF`

**Branch:** `main` (or create feature branch if preferred)

**Verify Rust tests pass:**
```bash
cd voxpdf-core
cargo test
```

Expected: All 14 tests passing

---

## Task 1: Complete Paragraphs FFI

**Goal:** Add FFI functions to extract paragraphs from PDF pages.

**Files:**
- Modify: `voxpdf-core/src/ffi.rs` (add paragraph functions)
- Test: `voxpdf-core/src/ffi.rs` (add test in tests module)

---

### Step 1: Define C paragraph structure

Add to `voxpdf-core/src/ffi.rs` after `CWordPosition`:

```rust
/// C-compatible paragraph structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CParagraph {
    pub index: usize,
    pub page_number: u32,
    pub word_count: usize,
}
```

**Run:** `cargo build`

**Expected:** Compiles successfully

---

### Step 2: Write failing FFI test for paragraphs

Add to `voxpdf-core/src/ffi.rs` in `mod tests`:

```rust
#[test]
fn test_ffi_paragraphs() {
    let path = CString::new("tests/fixtures/simple.pdf").unwrap();
    let mut error = CVoxPDFError::Ok;

    unsafe {
        let doc = voxpdf_open(path.as_ptr(), &mut error);
        assert!(!doc.is_null());

        let count = voxpdf_get_paragraph_count(doc, 0, &mut error);
        assert!(count > 0);
        assert_eq!(error, CVoxPDFError::Ok);

        let mut para = CParagraph {
            index: 0,
            page_number: 0,
            word_count: 0,
        };
        let mut text_ptr: *const c_char = std::ptr::null();

        let result = voxpdf_get_paragraph(doc, 0, 0, &mut para, &mut text_ptr, &mut error);
        assert!(result);
        assert!(para.word_count > 0);
        assert!(!text_ptr.is_null());

        voxpdf_free_string(text_ptr as *mut c_char);
        voxpdf_free_document(doc);
    }
}
```

---

### Step 3: Run test to verify it fails

**Run:** `cargo test test_ffi_paragraphs`

**Expected:** FAIL with "cannot find function `voxpdf_get_paragraph_count`"

---

### Step 4: Implement voxpdf_get_paragraph_count

Add to `voxpdf-core/src/ffi.rs` after `voxpdf_get_word`:

```rust
/// Get the number of paragraphs on a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_paragraph_count(
    doc: *const CVoxPDFDocument,
    page: u32,
    error_out: *mut CVoxPDFError,
) -> usize {
    if doc.is_null() || error_out.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);

    // Extract words and detect paragraphs
    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            let paragraphs = crate::extraction::detect_paragraphs(words);
            *error_out = CVoxPDFError::Ok;
            paragraphs.len()
        }
        Err(e) => {
            *error_out = e.into();
            0
        }
    }
}
```

---

### Step 5: Implement voxpdf_get_paragraph

Add to `voxpdf-core/src/ffi.rs` after `voxpdf_get_paragraph_count`:

```rust
/// Get a specific paragraph by index from a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `para_out` must be a valid mutable pointer to CParagraph
/// - `text_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned text pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_paragraph(
    doc: *const CVoxPDFDocument,
    page: u32,
    index: usize,
    para_out: *mut CParagraph,
    text_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || para_out.is_null() || text_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    // Extract words and detect paragraphs
    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            let paragraphs = crate::extraction::detect_paragraphs(words);

            if let Some(para) = paragraphs.get(index) {
                *para_out = CParagraph {
                    index: para.index,
                    page_number: para.page_number,
                    word_count: para.word_count(),
                };

                match CString::new(para.text.clone()) {
                    Ok(c_str) => {
                        *text_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                        return true;
                    }
                    Err(_) => {
                        *error_out = CVoxPDFError::InvalidText;
                        return false;
                    }
                }
            }

            *error_out = CVoxPDFError::PageNotFound;
            false
        }
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}
```

---

### Step 6: Run test to verify it passes

**Run:** `cargo test test_ffi_paragraphs`

**Expected:** PASS

---

### Step 7: Run all FFI tests

**Run:** `cargo test ffi`

**Expected:** All FFI tests pass (4 tests)

---

### Step 8: Commit

```bash
git add voxpdf-core/src/ffi.rs
git commit -m "feat(ffi): add paragraph extraction FFI

Add FFI functions for paragraph extraction:
- voxpdf_get_paragraph_count() to count paragraphs on page
- voxpdf_get_paragraph() to retrieve paragraph metadata and text
- CParagraph struct for cross-language paragraph data
- Test for paragraph FFI lifecycle"
```

---

## Task 2: Complete TOC FFI

**Goal:** Add FFI functions to extract table of contents from PDFs.

**Files:**
- Modify: `voxpdf-core/src/ffi.rs` (add TOC functions)
- Test: `voxpdf-core/src/ffi.rs` (add test in tests module)

---

### Step 1: Define C TOC entry structure

Add to `voxpdf-core/src/ffi.rs` after `CParagraph`:

```rust
/// C-compatible TOC entry structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CTocEntry {
    pub level: u8,
    pub page_number: u32,
    pub paragraph_index: usize,
}
```

**Run:** `cargo build`

**Expected:** Compiles successfully

---

### Step 2: Write failing FFI test for TOC

Add to `voxpdf-core/src/ffi.rs` in `mod tests`:

```rust
#[test]
fn test_ffi_toc() {
    let path = CString::new("tests/fixtures/simple.pdf").unwrap();
    let mut error = CVoxPDFError::Ok;

    unsafe {
        let doc = voxpdf_open(path.as_ptr(), &mut error);
        assert!(!doc.is_null());

        // simple.pdf has no TOC, so should return 0
        let count = voxpdf_get_toc_count(doc, &mut error);
        assert_eq!(count, 0);
        assert_eq!(error, CVoxPDFError::Ok);

        voxpdf_free_document(doc);
    }
}
```

---

### Step 3: Run test to verify it fails

**Run:** `cargo test test_ffi_toc`

**Expected:** FAIL with "cannot find function `voxpdf_get_toc_count`"

---

### Step 4: Implement voxpdf_get_toc_count

Add to `voxpdf-core/src/ffi.rs` after `voxpdf_get_paragraph`:

```rust
/// Get the number of TOC entries in the document.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_toc_count(
    doc: *const CVoxPDFDocument,
    error_out: *mut CVoxPDFError,
) -> usize {
    if doc.is_null() || error_out.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_toc(doc) {
        Ok(toc_entries) => {
            *error_out = CVoxPDFError::Ok;
            toc_entries.len()
        }
        Err(e) => {
            *error_out = e.into();
            0
        }
    }
}
```

---

### Step 5: Implement voxpdf_get_toc_entry

Add to `voxpdf-core/src/ffi.rs` after `voxpdf_get_toc_count`:

```rust
/// Get a specific TOC entry by index.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `toc_out` must be a valid mutable pointer to CTocEntry
/// - `title_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned title pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_toc_entry(
    doc: *const CVoxPDFDocument,
    index: usize,
    toc_out: *mut CTocEntry,
    title_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || toc_out.is_null() || title_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_toc(doc) {
        Ok(toc_entries) => {
            if let Some(entry) = toc_entries.get(index) {
                *toc_out = CTocEntry {
                    level: entry.level,
                    page_number: entry.page_number,
                    paragraph_index: entry.paragraph_index,
                };

                match CString::new(entry.title.clone()) {
                    Ok(c_str) => {
                        *title_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                        return true;
                    }
                    Err(_) => {
                        *error_out = CVoxPDFError::InvalidText;
                        return false;
                    }
                }
            }

            *error_out = CVoxPDFError::PageNotFound;
            false
        }
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}
```

---

### Step 6: Run test to verify it passes

**Run:** `cargo test test_ffi_toc`

**Expected:** PASS

---

### Step 7: Run all FFI tests

**Run:** `cargo test ffi`

**Expected:** All FFI tests pass (5 tests)

---

### Step 8: Commit

```bash
git add voxpdf-core/src/ffi.rs
git commit -m "feat(ffi): add TOC extraction FFI

Add FFI functions for TOC extraction:
- voxpdf_get_toc_count() to count TOC entries
- voxpdf_get_toc_entry() to retrieve TOC entry metadata
- CTocEntry struct for cross-language TOC data
- Test for TOC FFI (validates empty TOC case)"
```

---

## Task 3: Create Swift Package Structure

**Goal:** Set up the voxpdf-swift Swift package with proper structure and modulemap.

**Files:**
- Create: `voxpdf-swift/Package.swift`
- Create: `voxpdf-swift/Sources/VoxPDF/module.modulemap`
- Create: `voxpdf-swift/Sources/VoxPDF/include/voxpdf.h`
- Create: `voxpdf-swift/Tests/VoxPDFTests/VoxPDFTests.swift`

---

### Step 1: Create directory structure

**Run:**
```bash
mkdir -p voxpdf-swift/Sources/VoxPDF/include
mkdir -p voxpdf-swift/Tests/VoxPDFTests
```

**Expected:** Directories created

---

### Step 2: Create Package.swift

Create file: `voxpdf-swift/Package.swift`

```swift
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "VoxPDF",
    platforms: [
        .iOS(.v15),
        .macOS(.v12)
    ],
    products: [
        .library(
            name: "VoxPDF",
            targets: ["VoxPDF"]
        ),
    ],
    targets: [
        .target(
            name: "VoxPDF",
            dependencies: [],
            path: "Sources/VoxPDF"
        ),
        .testTarget(
            name: "VoxPDFTests",
            dependencies: ["VoxPDF"],
            path: "Tests/VoxPDFTests",
            resources: [
                .copy("Fixtures")
            ]
        ),
    ]
)
```

---

### Step 3: Create C header file

Create file: `voxpdf-swift/Sources/VoxPDF/include/voxpdf.h`

```c
#ifndef VOXPDF_H
#define VOXPDF_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Error codes
typedef enum {
    CVoxPDFErrorOk = 0,
    CVoxPDFErrorInvalidPDF = 1,
    CVoxPDFErrorPageNotFound = 2,
    CVoxPDFErrorIoError = 3,
    CVoxPDFErrorOutOfMemory = 4,
    CVoxPDFErrorInvalidText = 5,
} CVoxPDFError;

// Opaque document pointer
typedef struct CVoxPDFDocument CVoxPDFDocument;

// Word position structure
typedef struct {
    float x;
    float y;
    float width;
    float height;
    uint32_t page;
} CWordPosition;

// Paragraph structure
typedef struct {
    size_t index;
    uint32_t page_number;
    size_t word_count;
} CParagraph;

// TOC entry structure
typedef struct {
    uint8_t level;
    uint32_t page_number;
    size_t paragraph_index;
} CTocEntry;

// Document lifecycle
CVoxPDFDocument* voxpdf_open(const char* path, CVoxPDFError* error_out);
size_t voxpdf_get_page_count(const CVoxPDFDocument* doc);
void voxpdf_free_document(CVoxPDFDocument* doc);

// Text extraction
bool voxpdf_extract_page_text(
    const CVoxPDFDocument* doc,
    uint32_t page,
    const char** text_out,
    CVoxPDFError* error_out
);

// Word positions
size_t voxpdf_get_word_count(
    const CVoxPDFDocument* doc,
    uint32_t page,
    CVoxPDFError* error_out
);

bool voxpdf_get_word(
    const CVoxPDFDocument* doc,
    uint32_t page,
    size_t index,
    CWordPosition* word_out,
    const char** text_out,
    CVoxPDFError* error_out
);

// Paragraphs
size_t voxpdf_get_paragraph_count(
    const CVoxPDFDocument* doc,
    uint32_t page,
    CVoxPDFError* error_out
);

bool voxpdf_get_paragraph(
    const CVoxPDFDocument* doc,
    uint32_t page,
    size_t index,
    CParagraph* para_out,
    const char** text_out,
    CVoxPDFError* error_out
);

// TOC
size_t voxpdf_get_toc_count(
    const CVoxPDFDocument* doc,
    CVoxPDFError* error_out
);

bool voxpdf_get_toc_entry(
    const CVoxPDFDocument* doc,
    size_t index,
    CTocEntry* toc_out,
    const char** title_out,
    CVoxPDFError* error_out
);

// Memory management
void voxpdf_free_string(char* s);

#endif // VOXPDF_H
```

---

### Step 4: Create modulemap

Create file: `voxpdf-swift/Sources/VoxPDF/module.modulemap`

```
module CVoxPDF {
    header "include/voxpdf.h"
    link "voxpdf_core"
    export *
}
```

---

### Step 5: Create test fixture directory

**Run:**
```bash
mkdir -p voxpdf-swift/Tests/VoxPDFTests/Fixtures
cp voxpdf-core/tests/fixtures/simple.pdf voxpdf-swift/Tests/VoxPDFTests/Fixtures/
```

**Expected:** Test PDF copied

---

### Step 6: Verify Swift package builds

**Run:**
```bash
cd voxpdf-swift
swift build
```

**Expected:** May fail due to missing library link, but should parse Package.swift successfully

---

### Step 7: Commit

```bash
git add voxpdf-swift/
git commit -m "feat(swift): create Swift package structure

Add VoxPDF Swift package:
- Package.swift with iOS 15+ and macOS 12+ support
- C header with all FFI function declarations
- modulemap for CVoxPDF module
- Test directory with fixture
- Empty source and test directories"
```

---

## Task 4: Implement Swift PDFDocument Wrapper

**Goal:** Create the main Swift PDFDocument class that wraps the Rust FFI.

**Files:**
- Create: `voxpdf-swift/Sources/VoxPDF/VoxPDFError.swift`
- Create: `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift`
- Create: `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`

---

### Step 1: Create VoxPDFError enum

Create file: `voxpdf-swift/Sources/VoxPDF/VoxPDFError.swift`

```swift
import Foundation

/// Errors that can occur during PDF operations.
public enum VoxPDFError: Error {
    case invalidPDF
    case pageNotFound
    case ioError
    case outOfMemory
    case invalidText
    case unknown

    init(code: Int32) {
        switch code {
        case 1: self = .invalidPDF
        case 2: self = .pageNotFound
        case 3: self = .ioError
        case 4: self = .outOfMemory
        case 5: self = .invalidText
        default: self = .unknown
        }
    }
}
```

---

### Step 2: Write failing test for PDFDocument

Create file: `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`

```swift
import XCTest
@testable import VoxPDF

final class PDFDocumentTests: XCTestCase {
    let testPDFPath = "../../../voxpdf-core/tests/fixtures/simple.pdf"

    func testOpenDocument() throws {
        let url = URL(fileURLWithPath: testPDFPath)
        let doc = try PDFDocument(url: url)

        XCTAssertEqual(doc.pageCount, 1)
    }

    func testOpenNonexistentDocument() {
        let url = URL(fileURLWithPath: "/nonexistent.pdf")

        XCTAssertThrowsError(try PDFDocument(url: url)) { error in
            XCTAssertTrue(error is VoxPDFError)
        }
    }

    func testExtractText() throws {
        let url = URL(fileURLWithPath: testPDFPath)
        let doc = try PDFDocument(url: url)

        let text = try doc.text(page: 0)

        XCTAssertFalse(text.isEmpty)
        XCTAssertTrue(text.contains("Hello"))
    }
}
```

---

### Step 3: Run test to verify it fails

**Run:**
```bash
cd voxpdf-swift
swift test
```

**Expected:** FAIL with "cannot find type 'PDFDocument'"

---

### Step 4: Implement PDFDocument class

Create file: `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift`

```swift
import Foundation

/// A PDF document that provides text and structure extraction.
public class PDFDocument {
    private let handle: OpaquePointer

    /// The number of pages in the document.
    public var pageCount: Int {
        Int(voxpdf_get_page_count(handle))
    }

    /// Opens a PDF document from a file URL.
    ///
    /// - Parameter url: The file URL of the PDF to open
    /// - Throws: `VoxPDFError` if the PDF cannot be opened
    public init(url: URL) throws {
        var error: Int32 = 0

        guard let handle = voxpdf_open(url.path, &error) else {
            throw VoxPDFError(code: error)
        }

        if error != 0 {
            throw VoxPDFError(code: error)
        }

        self.handle = handle
    }

    deinit {
        voxpdf_free_document(handle)
    }

    /// Extracts the text content from a specific page.
    ///
    /// - Parameter page: The zero-indexed page number
    /// - Returns: The extracted text
    /// - Throws: `VoxPDFError` if extraction fails
    public func text(page: Int) throws -> String {
        var textPtr: UnsafePointer<CChar>?
        var error: Int32 = 0

        let result = voxpdf_extract_page_text(
            handle,
            UInt32(page),
            &textPtr,
            &error
        )

        guard result, error == 0, let ptr = textPtr else {
            throw VoxPDFError(code: error)
        }

        defer { voxpdf_free_string(UnsafeMutablePointer(mutating: ptr)) }

        return String(cString: ptr)
    }
}
```

---

### Step 5: Run tests (will fail due to missing library)

**Run:**
```bash
cd voxpdf-swift
swift build
```

**Expected:** Build may succeed if library path is configured, or fail with link error

**Note:** For tests to pass, you need to build the Rust library first:

```bash
cd ../voxpdf-core
cargo build --release
```

Then set library path for Swift:

```bash
cd ../voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test
```

---

### Step 6: Document build instructions

Create file: `voxpdf-swift/README.md`

```markdown
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
```

---

### Step 7: Commit

```bash
git add voxpdf-swift/
git commit -m "feat(swift): implement PDFDocument wrapper

Add Swift PDFDocument class:
- init(url:) to open PDFs
- pageCount property
- text(page:) to extract text
- Automatic memory management via deinit
- VoxPDFError enum for error handling
- Tests for basic operations
- README with build instructions"
```

---

## Task 5: Implement Swift Word Models

**Goal:** Add Swift models and API for word position extraction.

**Files:**
- Create: `voxpdf-swift/Sources/VoxPDF/Word.swift`
- Modify: `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift`
- Modify: `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`

---

### Step 1: Write failing test for word positions

Add to `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`:

```swift
func testWordPositions() throws {
    let url = URL(fileURLWithPath: testPDFPath)
    let doc = try PDFDocument(url: url)

    let words = try doc.words(page: 0)

    XCTAssertGreaterThan(words.count, 0)

    // All words should have valid bounds
    for word in words {
        XCTAssertFalse(word.text.isEmpty)
        XCTAssertGreaterThan(word.bounds.width, 0)
        XCTAssertGreaterThan(word.bounds.height, 0)
    }

    // Should find "Hello"
    let hello = words.first { $0.text.contains("Hello") }
    XCTAssertNotNil(hello)
}
```

---

### Step 2: Run test to verify it fails

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testWordPositions
```

**Expected:** FAIL with "value of type 'PDFDocument' has no member 'words'"

---

### Step 3: Create Word struct

Create file: `voxpdf-swift/Sources/VoxPDF/Word.swift`

```swift
import Foundation
import CoreGraphics

/// A single word extracted from a PDF page with its position.
public struct Word {
    /// The text content of the word.
    public let text: String

    /// The bounding rectangle of the word in PDF coordinates.
    public let bounds: CGRect

    /// The zero-indexed page number.
    public let pageNumber: Int

    init(text: String, cPosition: CWordPosition) {
        self.text = text
        self.bounds = CGRect(
            x: CGFloat(cPosition.x),
            y: CGFloat(cPosition.y),
            width: CGFloat(cPosition.width),
            height: CGFloat(cPosition.height)
        )
        self.pageNumber = Int(cPosition.page)
    }
}
```

---

### Step 4: Add words method to PDFDocument

Add to `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift` after `text(page:)`:

```swift
/// Extracts word positions from a specific page.
///
/// - Parameter page: The zero-indexed page number
/// - Returns: Array of words with their positions
/// - Throws: `VoxPDFError` if extraction fails
public func words(page: Int) throws -> [Word] {
    var error: Int32 = 0

    let count = voxpdf_get_word_count(handle, UInt32(page), &error)
    guard error == 0 else {
        throw VoxPDFError(code: error)
    }

    var words: [Word] = []
    words.reserveCapacity(count)

    for index in 0..<count {
        var cPosition = CWordPosition(x: 0, y: 0, width: 0, height: 0, page: 0)
        var textPtr: UnsafePointer<CChar>?

        let result = voxpdf_get_word(
            handle,
            UInt32(page),
            index,
            &cPosition,
            &textPtr,
            &error
        )

        guard result, error == 0, let ptr = textPtr else {
            throw VoxPDFError(code: error)
        }

        let text = String(cString: ptr)
        voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

        words.append(Word(text: text, cPosition: cPosition))
    }

    return words
}
```

---

### Step 5: Run test to verify it passes

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testWordPositions
```

**Expected:** PASS

---

### Step 6: Run all tests

**Run:**
```bash
swift test
```

**Expected:** All tests pass

---

### Step 7: Commit

```bash
git add voxpdf-swift/
git commit -m "feat(swift): add word position extraction

Add Word model and API:
- Word struct with text and CGRect bounds
- PDFDocument.words(page:) method
- Automatic memory management for word text
- Tests for word extraction and validation"
```

---

## Task 6: Implement Swift Paragraph Models

**Goal:** Add Swift models and API for paragraph extraction.

**Files:**
- Create: `voxpdf-swift/Sources/VoxPDF/Paragraph.swift`
- Modify: `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift`
- Modify: `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`

---

### Step 1: Write failing test for paragraphs

Add to `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`:

```swift
func testParagraphs() throws {
    let url = URL(fileURLWithPath: testPDFPath)
    let doc = try PDFDocument(url: url)

    let paragraphs = try doc.paragraphs(page: 0)

    XCTAssertGreaterThan(paragraphs.count, 0)

    // All paragraphs should have valid data
    for para in paragraphs {
        XCTAssertFalse(para.text.isEmpty)
        XCTAssertGreaterThan(para.wordCount, 0)
    }

    // First paragraph should contain text
    XCTAssertTrue(paragraphs[0].text.contains("Hello"))
}
```

---

### Step 2: Run test to verify it fails

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testParagraphs
```

**Expected:** FAIL with "value of type 'PDFDocument' has no member 'paragraphs'"

---

### Step 3: Create Paragraph struct

Create file: `voxpdf-swift/Sources/VoxPDF/Paragraph.swift`

```swift
import Foundation

/// A paragraph of text extracted from a PDF page.
public struct Paragraph {
    /// The index of this paragraph within the page.
    public let index: Int

    /// The text content of the paragraph.
    public let text: String

    /// The zero-indexed page number.
    public let pageNumber: Int

    /// The number of words in this paragraph.
    public let wordCount: Int

    init(text: String, cParagraph: CParagraph) {
        self.index = cParagraph.index
        self.text = text
        self.pageNumber = Int(cParagraph.page_number)
        self.wordCount = cParagraph.word_count
    }
}
```

---

### Step 4: Add paragraphs method to PDFDocument

Add to `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift` after `words(page:)`:

```swift
/// Extracts paragraphs from a specific page.
///
/// - Parameter page: The zero-indexed page number
/// - Returns: Array of paragraphs with their text
/// - Throws: `VoxPDFError` if extraction fails
public func paragraphs(page: Int) throws -> [Paragraph] {
    var error: Int32 = 0

    let count = voxpdf_get_paragraph_count(handle, UInt32(page), &error)
    guard error == 0 else {
        throw VoxPDFError(code: error)
    }

    var paragraphs: [Paragraph] = []
    paragraphs.reserveCapacity(count)

    for index in 0..<count {
        var cPara = CParagraph(index: 0, page_number: 0, word_count: 0)
        var textPtr: UnsafePointer<CChar>?

        let result = voxpdf_get_paragraph(
            handle,
            UInt32(page),
            index,
            &cPara,
            &textPtr,
            &error
        )

        guard result, error == 0, let ptr = textPtr else {
            throw VoxPDFError(code: error)
        }

        let text = String(cString: ptr)
        voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

        paragraphs.append(Paragraph(text: text, cParagraph: cPara))
    }

    return paragraphs
}
```

---

### Step 5: Run test to verify it passes

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testParagraphs
```

**Expected:** PASS

---

### Step 6: Run all tests

**Run:**
```bash
swift test
```

**Expected:** All tests pass

---

### Step 7: Commit

```bash
git add voxpdf-swift/
git commit -m "feat(swift): add paragraph extraction

Add Paragraph model and API:
- Paragraph struct with text and metadata
- PDFDocument.paragraphs(page:) method
- Automatic memory management for paragraph text
- Tests for paragraph extraction"
```

---

## Task 7: Implement Swift TOC Models

**Goal:** Add Swift models and API for table of contents extraction.

**Files:**
- Create: `voxpdf-swift/Sources/VoxPDF/TocEntry.swift`
- Modify: `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift`
- Modify: `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`

---

### Step 1: Write failing test for TOC

Add to `voxpdf-swift/Tests/VoxPDFTests/PDFDocumentTests.swift`:

```swift
func testTableOfContents() throws {
    let url = URL(fileURLWithPath: testPDFPath)
    let doc = try PDFDocument(url: url)

    let toc = try doc.tableOfContents()

    // simple.pdf has no TOC, so should be empty
    XCTAssertEqual(toc.count, 0)
}
```

---

### Step 2: Run test to verify it fails

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testTableOfContents
```

**Expected:** FAIL with "value of type 'PDFDocument' has no member 'tableOfContents'"

---

### Step 3: Create TocEntry struct

Create file: `voxpdf-swift/Sources/VoxPDF/TocEntry.swift`

```swift
import Foundation

/// An entry in the PDF's table of contents.
public struct TocEntry {
    /// The title of this TOC entry.
    public let title: String

    /// The nesting level (0 = chapter, 1 = section, etc.).
    public let level: Int

    /// The zero-indexed page number where this entry points.
    public let pageNumber: Int

    /// The paragraph index for navigation.
    public let paragraphIndex: Int

    /// Returns true if this is a top-level chapter (level 0).
    public var isChapter: Bool {
        level == 0
    }

    /// Returns true if this is a section (level 1).
    public var isSection: Bool {
        level == 1
    }

    init(title: String, cTocEntry: CTocEntry) {
        self.title = title
        self.level = Int(cTocEntry.level)
        self.pageNumber = Int(cTocEntry.page_number)
        self.paragraphIndex = cTocEntry.paragraph_index
    }
}
```

---

### Step 4: Add tableOfContents method to PDFDocument

Add to `voxpdf-swift/Sources/VoxPDF/PDFDocument.swift` after `paragraphs(page:)`:

```swift
/// Extracts the table of contents from the document.
///
/// - Returns: Array of TOC entries (empty if document has no TOC)
/// - Throws: `VoxPDFError` if extraction fails
public func tableOfContents() throws -> [TocEntry] {
    var error: Int32 = 0

    let count = voxpdf_get_toc_count(handle, &error)
    guard error == 0 else {
        throw VoxPDFError(code: error)
    }

    var entries: [TocEntry] = []
    entries.reserveCapacity(count)

    for index in 0..<count {
        var cToc = CTocEntry(level: 0, page_number: 0, paragraph_index: 0)
        var titlePtr: UnsafePointer<CChar>?

        let result = voxpdf_get_toc_entry(
            handle,
            index,
            &cToc,
            &titlePtr,
            &error
        )

        guard result, error == 0, let ptr = titlePtr else {
            throw VoxPDFError(code: error)
        }

        let title = String(cString: ptr)
        voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

        entries.append(TocEntry(title: title, cTocEntry: cToc))
    }

    return entries
}
```

---

### Step 5: Run test to verify it passes

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test --filter testTableOfContents
```

**Expected:** PASS

---

### Step 6: Run all tests

**Run:**
```bash
swift test
```

**Expected:** All tests pass (7 tests)

---

### Step 7: Commit

```bash
git add voxpdf-swift/
git commit -m "feat(swift): add table of contents extraction

Add TocEntry model and API:
- TocEntry struct with title and navigation metadata
- PDFDocument.tableOfContents() method
- isChapter and isSection convenience properties
- Automatic memory management for titles
- Tests for TOC extraction (empty case)"
```

---

## Task 8: Create iOS Build Scripts

**Goal:** Add scripts to build the Rust library for iOS targets.

**Files:**
- Create: `voxpdf-core/scripts/build-ios.sh`
- Create: `voxpdf-core/scripts/create-xcframework.sh`

---

### Step 1: Add iOS targets to Rust

**Run:**
```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
rustup target add x86_64-apple-ios
```

**Expected:** Targets installed

---

### Step 2: Create iOS build script

Create file: `voxpdf-core/scripts/build-ios.sh`

```bash
#!/bin/bash
set -e

echo "Building VoxPDF for iOS targets..."

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
```

---

### Step 3: Make script executable

**Run:**
```bash
chmod +x voxpdf-core/scripts/build-ios.sh
```

**Expected:** Script is executable

---

### Step 4: Test iOS build script

**Run:**
```bash
cd voxpdf-core
./scripts/build-ios.sh
```

**Expected:** All three iOS targets build successfully

---

### Step 5: Create XCFramework creation script

Create file: `voxpdf-core/scripts/create-xcframework.sh`

```bash
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
    -headers voxpdf-swift/Sources/VoxPDF/include \
    -library target/aarch64-apple-ios-sim/release/libvoxpdf_core.a \
    -headers voxpdf-swift/Sources/VoxPDF/include \
    -library target/x86_64-apple-ios/release/libvoxpdf_core.a \
    -headers voxpdf-swift/Sources/VoxPDF/include \
    -output build/VoxPDFCore.xcframework

echo "✅ XCFramework created at build/VoxPDFCore.xcframework"
```

---

### Step 6: Make script executable

**Run:**
```bash
chmod +x voxpdf-core/scripts/create-xcframework.sh
```

**Expected:** Script is executable

---

### Step 7: Document build process

Add to `voxpdf-swift/README.md` (append to existing content):

```markdown

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
```

---

### Step 8: Commit

```bash
git add voxpdf-core/scripts/ voxpdf-swift/README.md
git commit -m "feat(build): add iOS build scripts

Add iOS build automation:
- build-ios.sh to build for all iOS targets
- create-xcframework.sh to package as XCFramework
- Updated README with iOS build instructions
- Support for device (ARM64) and simulator (ARM64 + x86_64)"
```

---

## Task 9: Final Validation and Documentation

**Goal:** Verify all tests pass and documentation is complete.

**Files:**
- Modify: `README.md` (project root)
- Modify: `docs/PROJECT_STATUS.md`

---

### Step 1: Run all Rust tests

**Run:**
```bash
cd voxpdf-core
cargo test
cargo test --release
cargo clippy -- -D warnings
cargo fmt --check
```

**Expected:** All pass with no warnings

---

### Step 2: Run all Swift tests

**Run:**
```bash
cd voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test
```

**Expected:** All 7 tests pass

---

### Step 3: Update PROJECT_STATUS.md

Update `docs/PROJECT_STATUS.md` to reflect completion:

Change FFI section from:
```markdown
**FFI Layer** - ⚠️ PARTIALLY COMPLETE
```

To:
```markdown
**FFI Layer** - ✅ COMPLETE
- Basic lifecycle: open, page_count, free
- Text extraction: extract_page_text
- Word positions: get_word_count, get_word
- Paragraphs: get_paragraph_count, get_paragraph
- TOC: get_toc_count, get_toc_entry
- All FFI tests passing
```

Change Swift section from:
```markdown
### ❌ NOT STARTED - Swift Bindings (voxpdf-swift)
```

To:
```markdown
### ✅ COMPLETE - Swift Bindings (voxpdf-swift)

**Package Structure:** ✅ Complete
- Package.swift with iOS 15+ support
- C header and modulemap
- Test infrastructure

**Swift Models:** ✅ Complete
- PDFDocument class
- Word struct
- Paragraph struct
- TocEntry struct
- VoxPDFError enum

**Swift API:** ✅ Complete
- text(page:) - Extract text
- words(page:) - Extract word positions
- paragraphs(page:) - Extract paragraphs
- tableOfContents() - Extract TOC

**iOS Build:** ✅ Complete
- Build scripts for all iOS targets
- XCFramework creation script
```

Update overall progress from "60%" to "95%":
```markdown
**Current Phase**: v0.1.0 Implementation - 95% Complete
```

---

### Step 4: Update root README.md

Update the project README to reflect current status.

Find the status section and update to:

```markdown
## Status

**Phase:** v0.1.0 Implementation - 95% Complete

**What Works:**
- ✅ Rust Core: Text extraction, word positions, paragraphs, TOC
- ✅ FFI Layer: Complete C API for all extraction features
- ✅ Swift Bindings: Full iOS/macOS support
- ✅ iOS Build: XCFramework creation

**What's Left:**
- End-to-end integration testing
- Performance benchmarks
- Documentation polish
- Release prep (CHANGELOG, version tags)

**Next:** v0.2.0 - Multi-column layout support
```

---

### Step 5: Run final verification

**Run:**
```bash
# Verify Rust
cd voxpdf-core
cargo test --all-features

# Verify Swift
cd ../voxpdf-swift
export DYLD_LIBRARY_PATH=../voxpdf-core/target/release:$DYLD_LIBRARY_PATH
swift test

# Verify iOS builds
cd ../voxpdf-core
./scripts/build-ios.sh
```

**Expected:** All pass

---

### Step 6: Commit documentation updates

```bash
git add README.md docs/PROJECT_STATUS.md
git commit -m "docs: update project status to 95% complete

Update documentation to reflect FFI and Swift completion:
- FFI layer fully implemented and tested
- Swift bindings complete with all extraction features
- iOS build scripts and XCFramework support
- All tests passing (Rust + Swift)
- Ready for integration testing and release prep"
```

---

## Completion Checklist

### Implementation Complete
- [x] FFI: Paragraph extraction (`voxpdf_get_paragraph_count`, `voxpdf_get_paragraph`)
- [x] FFI: TOC extraction (`voxpdf_get_toc_count`, `voxpdf_get_toc_entry`)
- [x] Swift: Package structure (Package.swift, modulemap, headers)
- [x] Swift: PDFDocument wrapper class
- [x] Swift: Word model and API
- [x] Swift: Paragraph model and API
- [x] Swift: TocEntry model and API
- [x] iOS: Build scripts for all targets
- [x] iOS: XCFramework creation

### Testing Complete
- [x] All Rust FFI tests pass (5 tests)
- [x] All Swift tests pass (7 tests)
- [x] iOS builds succeed for all targets
- [x] No compiler warnings (Rust clippy clean)
- [x] Code formatted (Rust cargo fmt)

### Documentation Complete
- [x] Swift API documented with doc comments
- [x] README.md updated with build instructions
- [x] PROJECT_STATUS.md reflects completion
- [x] iOS build process documented

---

## Next Steps After Plan Completion

1. **Integration Testing** - Test full Rust → FFI → Swift pipeline with real PDFs
2. **Performance Benchmarks** - Measure extraction speed for various PDF sizes
3. **Memory Leak Testing** - Use Xcode Instruments to verify no leaks
4. **CHANGELOG.md** - Document all changes for v0.1.0 release
5. **Version Tag** - Create git tag `v0.1.0`
6. **Release** - Publish to GitHub releases

---

**Estimated Time:** 4-6 hours focused work
**Task Breakdown:** 9 tasks, 40 steps total
**Dependencies:** Rust toolchain, Swift 5.9+, Xcode (for iOS builds)

**Plan Status:** Ready for execution
**Created:** 2025-11-08
**Author:** VoxPDF Implementation Team
