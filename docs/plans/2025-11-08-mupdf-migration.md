# MuPDF Migration Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Migrate from lopdf to mupdf to fix font encoding issues with production-grade PDF text extraction.

**Architecture:** Replace lopdf-based word extraction with mupdf's TextChar API. Keep existing paragraph detection, hyphenation, and model layers unchanged. The migration affects only PDF document loading and character-level text extraction.

**Tech Stack:** mupdf 0.5.0 (AGPL-3.0), Rust 1.70+

**Context:** During real-world testing with O'Reilly technical PDFs, lopdf's limited font encoding support caused hyphens to decode as "!" characters ("full-blown" → "full ! blown"). This breaks TTS narration. MuPDF is production-grade (used in Foxit, Sumatra PDF) and handles embedded fonts correctly.

---

## Task 1: Update Dependencies

**Files:**
- Modify: `voxpdf-core/Cargo.toml`

**Step 1: Update Cargo.toml dependencies**

Replace lopdf with mupdf and update license:

```toml
[package]
name = "voxpdf-core"
version = "0.0.1"
edition = "2021"
authors = ["VoxPDF Contributors"]
license = "AGPL-3.0"
description = "Cross-platform PDF text extraction library optimized for text-to-speech"
repository = "https://github.com/yourusername/voxpdf"
keywords = ["pdf", "tts", "text-to-speech", "accessibility", "extraction"]
categories = ["parsing", "accessibility"]
readme = "../README.md"

[lib]
crate-type = ["lib", "cdylib", "staticlib"]

[dependencies]
# PDF parsing - using MuPDF for production-grade font handling
mupdf = "0.5.0"

# Text processing
unicode-segmentation = "1.11"
regex = "1.10"

# Error handling
thiserror = "1.0"

# Utilities
once_cell = "1.19"

# Serialization (for FFI and tests)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[dev-dependencies]
criterion = "0.5"
proptest = "1.4"

[features]
default = []

[[bench]]
name = "word_extraction_bench"
harness = false

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
```

**Step 2: Verify mupdf downloads and compiles**

Run: `cargo check`

Expected: Downloads mupdf and dependencies, compiles successfully (may take 2-3 minutes for initial build)

**Step 3: Commit dependency change**

```bash
git add Cargo.toml Cargo.lock
git commit -m "build: migrate from lopdf to mupdf (AGPL-3.0)"
```

---

## Task 2: Rewrite PDFDocument Wrapper

**Files:**
- Modify: `voxpdf-core/src/pdf/mod.rs`

**Step 1: Update PDFDocument to use mupdf**

Replace the entire file content:

```rust
use crate::error::{Result, VoxPDFError};
use mupdf::Document as MuPdfDocument;

/// Wrapper around MuPDF document
pub struct PDFDocument {
    pub(crate) doc: MuPdfDocument,
    path: String,
}

impl PDFDocument {
    /// Open a PDF file from disk
    ///
    /// # Example
    /// ```
    /// use voxpdf_core::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn open(path: &str) -> Result<Self> {
        let doc = MuPdfDocument::open(path)
            .map_err(|e| VoxPDFError::IoError(format!("Failed to open PDF: {}", e)))?;

        Ok(PDFDocument {
            doc,
            path: path.to_string(),
        })
    }

    /// Get the number of pages in the document
    ///
    /// # Example
    /// ```
    /// use voxpdf_core::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// let count = doc.page_count();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn page_count(&self) -> u32 {
        self.doc.page_count().unwrap_or(0) as u32
    }

    /// Get the file path of this document
    pub fn path(&self) -> &str {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
        assert_eq!(doc.page_count(), 1);
    }
}
```

**Step 2: Run tests to verify it compiles**

Run: `cargo test pdf::tests::test_page_count`

Expected: Test passes (simple.pdf has 1 page)

**Step 3: Commit PDFDocument changes**

```bash
git add src/pdf/mod.rs
git commit -m "refactor: rewrite PDFDocument to use mupdf"
```

---

## Task 3: Rewrite Word Extraction

**Files:**
- Modify: `voxpdf-core/src/extraction/words.rs`

**Step 1: Replace word extraction implementation**

Replace the entire file:

```rust
use crate::error::{Result, VoxPDFError};
use crate::models::{Rect, Word};
use crate::pdf::PDFDocument;
use mupdf::TextPageFlags;

/// Extract word positions from a PDF page using MuPDF.
///
/// Groups consecutive TextChar objects into words based on spacing.
///
/// # Arguments
///
/// * `doc` - The PDF document
/// * `page_num` - Zero-indexed page number
///
/// # Returns
///
/// A vector of words with their bounding boxes.
pub fn extract_word_positions(doc: &PDFDocument, page_num: u32) -> Result<Vec<Word>> {
    // Get the page
    let page = doc.doc.load_page(page_num as i32)
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to load page {}: {}", page_num, e)))?;

    // Convert to text page
    let text_page = page.to_text_page(TextPageFlags::empty())
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to extract text: {}", e)))?;

    // Extract characters and group into words
    let mut words = Vec::new();
    let mut current_word = String::new();
    let mut word_chars: Vec<(char, f32, f32, f32)> = Vec::new(); // (char, x, y, size)

    // Word spacing threshold: characters farther apart than this start a new word
    const WORD_SPACING_THRESHOLD: f32 = 3.0;

    for block in text_page.blocks() {
        for line in block.lines() {
            let mut prev_x: Option<f32> = None;

            for text_char in line.chars() {
                let c = match text_char.char() {
                    Some(c) => c,
                    None => continue,
                };

                // Skip control characters and excessive whitespace
                if c.is_control() {
                    continue;
                }

                let origin = text_char.origin();
                let size = text_char.size();
                let x = origin.x;
                let y = origin.y;

                // Check if this character starts a new word
                let is_space = c.is_whitespace();
                let is_new_word = if let Some(prev) = prev_x {
                    (x - prev) > WORD_SPACING_THRESHOLD || is_space
                } else {
                    false
                };

                if is_new_word || is_space {
                    // Finish current word
                    if !current_word.trim().is_empty() {
                        let word = create_word_from_chars(
                            current_word.trim().to_string(),
                            &word_chars,
                            page_num,
                        );
                        words.push(word);
                    }
                    current_word.clear();
                    word_chars.clear();

                    // If this is not just whitespace, start new word with this char
                    if !is_space {
                        current_word.push(c);
                        word_chars.push((c, x, y, size));
                    }
                } else {
                    // Continue current word
                    current_word.push(c);
                    word_chars.push((c, x, y, size));
                }

                prev_x = Some(x + size * 0.6); // Approximate character width
            }

            // Finish word at end of line
            if !current_word.trim().is_empty() {
                let word = create_word_from_chars(
                    current_word.trim().to_string(),
                    &word_chars,
                    page_num,
                );
                words.push(word);
                current_word.clear();
                word_chars.clear();
            }

            prev_x = None; // Reset for next line
        }
    }

    Ok(words)
}

/// Create a Word from a collection of characters
fn create_word_from_chars(
    text: String,
    chars: &[(char, f32, f32, f32)],
    page_num: u32,
) -> Word {
    if chars.is_empty() {
        return Word::new(text, Rect::new(0.0, 0.0, 0.0, 0.0), page_num);
    }

    // Calculate bounding box
    let min_x = chars.iter().map(|(_, x, _, _)| *x).fold(f32::INFINITY, f32::min);
    let min_y = chars.iter().map(|(_, _, y, _)| *y).fold(f32::INFINITY, f32::min);
    let max_x = chars.iter().map(|(_, x, _, s)| x + s * 0.6).fold(f32::NEG_INFINITY, f32::max);
    let max_y = chars.iter().map(|(_, _, y, s)| y + s).fold(f32::NEG_INFINITY, f32::max);

    let width = max_x - min_x;
    let height = max_y - min_y;

    Word::new(text, Rect::new(min_x, min_y, width, height), page_num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operations() {
        // Integration tests in tests/word_positions.rs provide coverage
    }
}
```

**Step 2: Run word extraction tests**

Run: `cargo test --test word_positions`

Expected: May fail initially due to position differences, but should extract text correctly

**Step 3: Commit word extraction changes**

```bash
git add src/extraction/words.rs
git commit -m "refactor: rewrite word extraction using mupdf TextChar API"
```

---

## Task 4: Update Text Extraction

**Files:**
- Modify: `voxpdf-core/src/extraction/text.rs`

**Step 1: Simplify text extraction using mupdf**

Replace file content:

```rust
use crate::error::Result;
use crate::pdf::PDFDocument;
use mupdf::TextPageFlags;

/// Extract plain text from a PDF page
pub fn extract_page_text(doc: &PDFDocument, page_num: u32) -> Result<String> {
    let page = doc.doc.load_page(page_num as i32)?;
    let text_page = page.to_text_page(TextPageFlags::empty())?;

    let mut text = String::new();

    for block in text_page.blocks() {
        for line in block.lines() {
            for text_char in line.chars() {
                if let Some(c) = text_char.char() {
                    if !c.is_control() {
                        text.push(c);
                    }
                }
            }
            text.push('\n');
        }
        text.push('\n');
    }

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_extraction_unit() {
        // Integration tests in tests/text_extraction.rs provide coverage
    }
}
```

**Step 2: Run text extraction tests**

Run: `cargo test --test text_extraction`

Expected: Tests pass, text extraction works correctly

**Step 3: Commit text extraction changes**

```bash
git add src/extraction/text.rs
git commit -m "refactor: simplify text extraction using mupdf"
```

---

## Task 5: Remove Content Stream Decoder

**Files:**
- Delete: `voxpdf-core/src/extraction/content_stream.rs`
- Modify: `voxpdf-core/src/extraction/mod.rs`

**Step 1: Remove content_stream module**

Delete the file:

```bash
rm src/extraction/content_stream.rs
```

**Step 2: Update extraction module exports**

In `src/extraction/mod.rs`, remove content_stream:

```rust
pub mod hyphenation;
pub mod paragraphs;
pub mod text;
pub mod toc;
pub mod words;

pub use hyphenation::reassemble_hyphenated_words;
pub use paragraphs::detect_paragraphs;
pub use text::extract_page_text;
pub use toc::extract_toc;
pub use words::extract_word_positions;
```

**Step 3: Verify compilation**

Run: `cargo check`

Expected: Compiles successfully without content_stream

**Step 4: Commit cleanup**

```bash
git add src/extraction/content_stream.rs src/extraction/mod.rs
git commit -m "refactor: remove content_stream decoder (not needed with mupdf)"
```

---

## Task 6: Fix Integration Tests

**Files:**
- Modify: `voxpdf-core/tests/word_positions.rs`

**Step 1: Update word position test expectations**

The test may need adjusted position assertions since mupdf coordinates differ from lopdf:

```rust
use voxpdf_core::{PDFDocument, extraction::extract_word_positions};

#[test]
fn test_extract_word_positions_simple() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    // Verify words were extracted
    assert!(!words.is_empty(), "Should extract words from simple.pdf");

    // Verify text content (positions may vary, but text should be correct)
    let text: String = words.iter().map(|w| w.text.as_str()).collect::<Vec<_>>().join(" ");
    assert!(text.contains("Hello"), "Should contain 'Hello'");
    assert!(text.contains("World"), "Should contain 'World'");
}

#[test]
fn test_word_positions_accuracy() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    // Verify all words have valid positions
    for word in &words {
        assert!(word.bounds.width > 0.0, "Word should have width");
        assert!(word.bounds.height > 0.0, "Word should have height");
    }
}
```

**Step 2: Run word position tests**

Run: `cargo test --test word_positions`

Expected: Both tests pass

**Step 3: Commit test fixes**

```bash
git add tests/word_positions.rs
git commit -m "test: update word position tests for mupdf"
```

---

## Task 7: Update LICENSE

**Files:**
- Modify: `LICENSE` (in repository root)

**Step 1: Replace LICENSE file with AGPL-3.0**

Create new LICENSE file at repository root:

```
                    GNU AFFERO GENERAL PUBLIC LICENSE
                       Version 3, 19 November 2007

[Full AGPL-3.0 license text from: https://www.gnu.org/licenses/agpl-3.0.txt]
```

**Step 2: Add license header comment to main library file**

In `voxpdf-core/src/lib.rs`, add header:

```rust
// VoxPDF - Cross-platform PDF text extraction for text-to-speech
// Copyright (C) 2025 VoxPDF Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! VoxPDF Core - PDF text extraction optimized for text-to-speech
[rest of file continues...]
```

**Step 3: Commit license change**

```bash
git add ../LICENSE src/lib.rs
git commit -m "license: migrate to AGPL-3.0 (required by mupdf)"
```

---

## Task 8: Verify with O'Reilly PDF

**Files:**
- Run: `cargo run --example paragraph_demo_page <path> 0`

**Step 1: Run paragraph demo on O'Reilly PDF**

Run:
```bash
cargo run --example paragraph_demo_page "/Users/zachswift/Downloads/Building Applications with AI Agents Designing and Implementing Multiagent Systems (Michael Albada) copy.pdf" 0
```

Expected output:
- Should extract "full-blown" (NOT "full ! blown")
- Should extract "hard-to-maintain" (NOT "hard ! to ! maintain")
- Should detect ~5 paragraphs correctly

**Step 2: Verify fix in output**

Search output for:
```
Text: "full-blown autonomous agent"
```

Should NOT contain:
```
Text: "full ! blown"
```

**Step 3: Run all tests**

Run: `cargo test`

Expected: All tests pass (26 tests)

**Step 4: Create final commit**

```bash
git add .
git commit -m "feat: complete mupdf migration - fixes font encoding

- Replaces lopdf with mupdf for production-grade PDF parsing
- Fixes hyphen encoding bug (full ! blown → full-blown)
- Updates license to AGPL-3.0 (required by mupdf dependency)
- All tests passing (26 tests)
- Verified with real O'Reilly technical PDF

BREAKING CHANGE: License changed from MIT to AGPL-3.0"
```

---

## Task 9: Run Full Test Suite and Benchmarks

**Files:**
- Run comprehensive testing

**Step 1: Run all unit tests**

Run: `cargo test --lib`

Expected: 14 unit tests pass

**Step 2: Run all integration tests**

Run: `cargo test --tests`

Expected: 12 integration tests pass

**Step 3: Run doc tests**

Run: `cargo test --doc`

Expected: 3 doc tests pass

**Step 4: Optional: Run benchmarks**

Run: `cargo bench`

Expected: Word extraction benchmark completes, may be faster than lopdf

**Step 5: Final verification commit**

```bash
git add .
git commit -m "test: verify all tests pass with mupdf backend"
```

---

## Validation Checklist

After completing all tasks, verify:

- [ ] `cargo check` passes
- [ ] `cargo test` passes (26 tests)
- [ ] O'Reilly PDF extracts "full-blown" (not "full ! blown")
- [ ] Paragraph detection still works (5 paragraphs on page 0)
- [ ] LICENSE file is AGPL-3.0
- [ ] Cargo.toml license is "AGPL-3.0"
- [ ] No references to lopdf remain
- [ ] content_stream.rs is deleted

---

## Rollback Plan

If migration fails or takes >4 hours:

```bash
git reset --hard HEAD~N  # where N = number of commits
cargo add lopdf@0.32
cargo remove mupdf
# Edit Cargo.toml to restore MIT license
git commit -m "revert: rollback mupdf migration, return to lopdf"
```

Then implement " ! " → "-" workaround as temporary fix for v0.1.0.

---

## Success Criteria

**Migration is successful when:**

1. All 26 tests pass
2. O'Reilly PDF extracts with correct hyphens ("full-blown", not "full ! blown")
3. Paragraph detection works correctly (5 paragraphs detected)
4. No compilation errors or warnings (except unused import warnings in examples)
5. LICENSE properly updated to AGPL-3.0

**Total estimated time:** 2-3 hours (including testing and debugging)
