# Decision: Migrate from lopdf to mupdf

**Date:** 2025-11-08

**Status:** Accepted

## Context

VoxPDF v0.1.0 was initially implemented using `lopdf` (v0.32), a pure Rust PDF parsing library. During real-world testing with O'Reilly technical PDFs, we discovered a critical font encoding limitation:

**Problem:** lopdf has limited font encoding support. Characters in PDFs with custom embedded fonts decode incorrectly:
- "full-blown" → "full ! blown"
- "hard-to-maintain" → "hard ! to ! maintain"
- "ML-based" → "ML ! based"

**Impact:** For VoxPDF's TTS use case, this makes audio output completely unusable. A narrator would read "hard exclamation point to exclamation point maintain" instead of "hard-to-maintain".

## Decision

**Migrate to `mupdf` (v0.5.0)** - Rust bindings to MuPDF, the production-grade PDF library used in Foxit, Sumatra PDF, and other professional PDF viewers.

## Rationale

### Why mupdf?

✅ **Production-grade font handling** - MuPDF properly decodes embedded fonts, ligatures, and complex encodings
✅ **Proven reliability** - Battle-tested in commercial PDF viewers
✅ **Aligns with project goals** - VoxPDF aims to be "as robust as Foxit eventually"
✅ **Compatible API** - TextChar with coordinates similar to our current design
✅ **Preserves existing work** - Paragraph detection, hyphenation, models all stay the same

### License Change

- **Current:** MIT License
- **New:** AGPL-3.0 License (required by mupdf-rs)

**Implication:** Applications using VoxPDF must also be open-source under AGPL-3.0 terms. This is acceptable since VoxPDF will always remain open-source.

### Alternatives Considered

1. **Stay with lopdf + workaround** - Replace " ! " with "-" as post-processing
   - ❌ Fragile heuristic, won't fix all encoding issues
   - ❌ Real exclamation marks could be corrupted
   - ❌ Doesn't solve underlying font problem

2. **pdf-extract crate** - Higher-level extraction library
   - ❌ Uses lopdf under the hood, same encoding issues

3. **pdfium** - Google's PDF library (used in Chrome)
   - ❌ Rust bindings not found or immature

## Migration Plan

### Phase 1: Core Migration
1. Update `Cargo.toml`: Add `mupdf = "0.5.0"`, remove `lopdf`
2. Rewrite `src/pdf/mod.rs` to use `mupdf::Document`
3. Rewrite `src/extraction/words.rs` to use `TextChar` API
4. Update content_stream decoder (may not be needed with mupdf)

### Phase 2: Fix Tests
1. Update `tests/word_positions.rs`
2. Update `tests/text_extraction.rs`
3. Update `tests/paragraph_detection.rs`
4. Fix FFI tests

### Phase 3: License & Documentation
1. Update `LICENSE` file to AGPL-3.0
2. Update `README.md` to mention AGPL
3. Update `Cargo.toml` license field
4. Document breaking change in v0.1.0 release notes

## Implementation Details

### mupdf-rs API

**Document Opening:**
```rust
use mupdf::Document;

let document = Document::open(&filename)?;
```

**Text Extraction Hierarchy:**
```rust
for page in document.pages()? {
    let text_page = page?.to_text_page(TextPageFlags::empty())?;

    for block in text_page.blocks() {
        for line in block.lines() {
            for char in line.chars() {
                let c = char.char().unwrap();      // Unicode character
                let origin = char.origin();         // Point { x, y }
                let quad = char.quad();             // Bounding box
                let size = char.size();             // Font size
            }
        }
    }
}
```

**Word Grouping Strategy:**
- Iterate through `TextChar` objects
- Group consecutive chars with small spacing into words
- Use `origin()` for position, `quad()` for bounds
- Keep existing paragraph detection algorithm (works on Word objects)

## Consequences

### Positive
- ✅ Fixes font encoding bug completely
- ✅ Production-grade text extraction
- ✅ Aligns with long-term robustness goals
- ✅ Most code preserved (models, paragraph logic, hyphenation)

### Negative
- ⚠️ License change to AGPL-3.0 (users must open-source their apps)
- ⚠️ Larger binary size (~10MB for libmupdf)
- ⚠️ FFI complexity (unsafe Rust bindings to C library)
- ⚠️ Migration effort for v0.1.0 implementation

### Neutral
- 🔄 Better text extraction may require re-tuning paragraph detection thresholds
- 🔄 Performance characteristics may differ (likely faster, needs benchmarking)

## Testing Strategy

1. Run existing test suite with mupdf backend
2. Verify O'Reilly PDF extracts "full-blown" correctly (not "full ! blown")
3. Run paragraph demo on real PDFs
4. Benchmark performance vs lopdf (optional)

## Rollback Plan

If mupdf migration fails or takes >2 days:
- Revert to lopdf
- Ship v0.1.0 with " ! " → "-" workaround
- Document font encoding limitation
- Plan mupdf for v0.2.0

---

**Approved by:** @zachswift
**Implementation:** In progress
