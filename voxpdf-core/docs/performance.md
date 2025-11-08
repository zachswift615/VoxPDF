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

## Future Optimizations

- Cache parsed content streams
- Parallel page processing for multi-page PDFs
- SIMD for text processing
- Pre-compile regex patterns
