use voxpdf_core::extraction::extract_toc;
use voxpdf_core::PDFDocument;

#[test]
#[cfg(feature = "toc-fallback")]
fn test_extract_toc_ai_agents_book_with_fallback() {
    // This PDF has a corrupted outline that mupdf rejects (error code 7),
    // but lopdf can parse it successfully. Tests multi-parser fallback.
    let doc = PDFDocument::open("tests/fixtures/ai-agents-book.pdf").unwrap();
    let toc_entries = extract_toc(&doc).unwrap();

    // With lopdf fallback, we successfully extract the TOC
    assert!(
        !toc_entries.is_empty(),
        "Should extract TOC via lopdf fallback"
    );
    assert!(
        toc_entries.len() > 200,
        "Book should have substantial TOC (238 entries)"
    );

    // Verify hierarchy
    let top_level: Vec<_> = toc_entries.iter().filter(|e| e.level == 0).collect();
    assert!(
        top_level.len() >= 15,
        "Should have multiple top-level chapters"
    );

    // Verify first entry
    assert!(
        toc_entries[0].title.contains("Copyright") || toc_entries[0].title.contains("Table"),
        "First entry should be Copyright or TOC"
    );
}

#[test]
#[cfg(not(feature = "toc-fallback"))]
fn test_extract_toc_ai_agents_book_no_fallback() {
    // Without fallback feature, corrupted outlines return empty
    let doc = PDFDocument::open("tests/fixtures/ai-agents-book.pdf").unwrap();
    let toc_entries = extract_toc(&doc).unwrap();
    assert_eq!(
        toc_entries.len(),
        0,
        "Without fallback, corrupted outline returns empty"
    );
}

#[test]
fn test_extract_toc_empty() {
    // simple.pdf has no outline
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let toc_entries = extract_toc(&doc).unwrap();

    assert_eq!(toc_entries.len(), 0);
}

#[test]
fn test_extract_toc_simple() {
    // toc-test.pdf has a known structure: 4 top-level + 1 nested = 5 total
    let doc = PDFDocument::open("tests/fixtures/toc-test.pdf").unwrap();
    let toc_entries = extract_toc(&doc).unwrap();

    assert_eq!(toc_entries.len(), 5, "Should have 5 total entries");

    // Check top-level chapters (level 0)
    let top_level: Vec<_> = toc_entries.iter().filter(|e| e.level == 0).collect();
    assert_eq!(top_level.len(), 4, "Should have 4 top-level chapters");

    // Check nested sections (level 1)
    let nested: Vec<_> = toc_entries.iter().filter(|e| e.level == 1).collect();
    assert_eq!(nested.len(), 1, "Should have 1 nested section");

    // Verify first entry
    assert_eq!(toc_entries[0].title, "Chapter 1: Introduction");
    assert_eq!(toc_entries[0].level, 0);
    assert_eq!(toc_entries[0].page_number, 0); // Page numbers are 0-indexed in mupdf
    assert!(
        toc_entries[0].is_chapter(),
        "First entry should be a chapter"
    );
}
