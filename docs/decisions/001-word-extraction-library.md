# ADR 001: Word Extraction Library Choice

**Status:** Accepted
**Date:** 2025-11-07
**Spike Duration:** ~3 hours
**Decision:** Continue with lopdf + custom workarounds

## Context

VoxPDF v0.1.0 requires extracting word-level position information from PDFs for TTS synchronization. The critical spike (Task 0.4) tested whether lopdf could extract word positions with <10% error.

## Decision Criteria

- ✅ **PASS**: Word positions extracted with <10% error → Continue with lopdf
- ⚠️ **PARTIAL**: Can extract text regions but not word-level bounds → Ship v0.1.0 without word positions
- ❌ **FAIL**: Cannot extract positions OR takes >8 hours → Pivot to mupdf-sys

## Findings

### Test Results: **PASS** ✅

Successfully extracted word positions from `simple.pdf` with **0% position error**:

| Word  | Expected Position | Actual Position | Error |
|-------|-------------------|-----------------|-------|
| Hello | (100, 592)        | (100, 592)      | 0%    |
| World | (160, 592)        | (160, 592)      | 0%    |

Width estimation: ~30pt (5 chars × 12pt font × 0.5 multiplier)
- Expected: 27.3pt (Hello), 31.3pt (World)
- Accuracy: Within 10% for width estimation

### Implementation Details

**What Works:**
- lopdf 0.32 can successfully decode PDF content streams
- PDF operator parsing (BT, ET, Tm, Tj, Tf) works correctly
- Position extraction from Tm (text matrix) operator is accurate
- Font size extraction from Tf operator works

**Critical Bug Discovered:**
- lopdf 0.32's `get_and_decode_page_content()` **fails** with ASCII85+Flate filter combinations
- Returns corrupt operators ("GapQh", "E") instead of valid PDF operators
- Common in ReportLab-generated PDFs

**Workaround Implemented:**
- Manual stream decoding using `ascii85` + `flate2` crates
- Detects Filter: [/ASCII85Decode /FlateDecode] in stream dictionary
- Manually decodes: raw → ASCII85 decode → Zlib decompress → text
- Falls back to lopdf's native decoding for other filter types

### Code Structure

```
voxpdf-core/src/extraction/
├── mod.rs                  # Public API exports
└── words.rs                # Word position extraction
    ├── extract_word_positions()   # Main entry point
    ├── decode_content_stream()    # Stream decoding (with workarounds)
    └── parse_content_text()       # PDF operator parsing
```

Dependencies added:
```toml
flate2 = "1.0"      # Zlib decompression
ascii85 = "0.1"     # ASCII85 decoding
```

### Limitations & Notes

1. **Width Estimation**: Character width is estimated (char_count × font_size × 0.5)
   - Accurate enough for TTS word highlighting
   - Could be improved with font metrics parsing (future enhancement)

2. **Supported Operators** (current implementation):
   - `Tm`: Text matrix (absolute positioning)
   - `Tj`: Show text
   - `Tf`: Font selection
   - `Td`, `TD`: Relative text positioning (implemented but not tested)

3. **Not Yet Supported** (may need for complex PDFs):
   - `TJ`: Show text with individual glyph positioning
   - `T*`: Move to next line
   - Font encoding/character mapping
   - Multi-byte character sets
   - Rotated text (non-identity text matrix)

4. **lopdf Limitations**:
   - ASCII85+Flate decoding bug requires workaround
   - No built-in text extraction with positions
   - Requires manual content stream parsing

## Decision

**Continue with lopdf** ✅

### Rationale

1. **Spike passed**: 0% position error meets <10% requirement
2. **Pure Rust**: No FFI complexity, easier cross-platform support
3. **Workaround viable**: ASCII85+Flate fix is ~50 lines of code
4. **Sufficient for v0.1.0**: Simple PDFs work, complex cases can be added incrementally

### Trade-offs Accepted

- **Pro**: No native dependencies, easier to build/deploy
- **Pro**: Full control over parsing logic
- **Con**: Manual content stream parsing required
- **Con**: May need more workarounds for complex PDFs
- **Con**: Width estimation is approximate (not font-metric-based)

## Alternatives Considered

### Option B: mupdf-sys

**Pros:**
- Battle-tested PDF rendering engine
- Accurate font metrics and character positioning
- Handles complex PDFs (rotated text, CJK, etc.)

**Cons:**
- FFI complexity (C bindings)
- Native dependency complicates cross-platform builds
- Larger binary size
- Not pure Rust

**Decision:** Keep as fallback option if lopdf limitations become blocker

## Future Improvements

1. **Font Metrics**: Parse font dictionaries for accurate character widths
2. **TJ Operator**: Support individual glyph positioning
3. **Character Mapping**: Handle font encoding for non-ASCII text
4. **Rotated Text**: Parse full text matrix for transformations
5. **Upstream Contribution**: Report ASCII85+Flate bug to lopdf maintainers

## References

- Spike Test: `voxpdf-core/tests/word_position_spike.rs`
- Implementation: `voxpdf-core/src/extraction/words.rs`
- Test Fixture: `voxpdf-core/tests/fixtures/simple.pdf`
- Expected Positions: `voxpdf-core/tests/fixtures/simple-positions.json`

## Validation

```bash
# Run spike test
cargo test spike_validates_word_positions

# Expected: PASS (0% error on position extraction)
```

**Result:** ✅ Test passes - positions extracted with 0% error
