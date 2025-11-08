# VoxPDF Performance Benchmarks

## Word Position Extraction

**Simple PDF (1 page, 2 words):**
- Time: 5.7375 µs (0.0057 ms) per iteration
- Target: <50ms per page

**Status:** PASS (8,712x faster than target)

**Test Environment:**
- CPU: automatically detected by criterion
- OS: macOS (Darwin 23.6.0)
- Date: 2025-11-08

## Optimizations Applied

- None yet (baseline implementation)
- Pure lopdf extraction with ASCII85+Flate workaround

## Known Limitations

### Width Estimation (v0.1.0)

**Issue:** Word widths are estimated using a fixed multiplier (0.5 × font_size × character_count).

**Assumptions:**
- Proportional font (Helvetica, Arial, Times)
- Average character width is 50% of font height

**Inaccuracies:**
- **Monospace fonts** (Courier, Monaco) → estimated width too wide
- **Condensed fonts** (Arial Narrow) → estimated width too wide
- **Wide fonts** → estimated width too narrow
- **Kerning** → not accounted for

**Impact:**
- TTS highlighting boxes may not perfectly align with text
- Acceptable for v0.1.0 where approximate bounds are sufficient

**Proper Solution (v0.2.0):**
1. Parse font metrics from PDF font dictionary (`/Widths` array)
2. Look up actual character widths per glyph
3. Apply kerning adjustments from font data
4. Account for font transformations (scaling, skewing)

**Reference:** `src/extraction/words.rs:101-119`

---

## Known Performance Issues

### FFI Word Extraction (v0.1.0)

**Issue:** The current FFI API extracts all words on every `voxpdf_get_word()` call, resulting in O(N²) behavior.

**Example:** For a page with 100 words:
- Swift calls `voxpdf_get_word_count()` → 1 extraction
- Swift calls `voxpdf_get_word()` 100 times → 100 extractions
- **Total: 101 extractions instead of 1**

**Performance Impact:**
- Single extraction: ~5.7µs
- 101 extractions: ~577µs (still 86x under 50ms target)
- **Status: Acceptable for v0.1.0**

**Root Cause:** Stateless FFI design - each call is independent, no caching across calls.

**Future Fix (v0.2.0):**
- Add batch API: `voxpdf_get_all_words()` that returns array
- Requires managing heap-allocated arrays across FFI boundary
- Or add session state with cached extraction results

## Future Optimizations

- **Batch word extraction API** (eliminates O(N²) FFI overhead)
- Cache parsed content streams per document
- Parallel page processing for multi-page PDFs
- SIMD for text processing
- Pre-compile regex patterns
- Font metrics integration for accurate width calculation
