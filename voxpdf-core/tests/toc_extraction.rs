use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_toc;

#[test]
#[ignore] // AI Agents book PDF has corrupted outline structure
fn test_extract_toc_ai_agents_book() {
    // Note: This PDF reports "Outline last pointer still bad or missing despite repair"
    // from mupdf, indicating the outline structure is corrupted. The PDF may have
    // bookmarks visible in some viewers, but mupdf cannot parse them.
    let doc = PDFDocument::open("tests/fixtures/ai-agents-book.pdf").unwrap();
    let chapters = extract_toc(&doc).unwrap();

    // Due to corrupted outline, this returns empty
    // If the PDF outline were valid, we would expect chapters here
    assert_eq!(chapters.len(), 0, "Corrupted outline should return empty TOC");
}

#[test]
fn test_extract_toc_empty() {
    // simple.pdf has no outline
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let chapters = extract_toc(&doc).unwrap();

    assert_eq!(chapters.len(), 0);
}

#[test]
fn test_extract_toc_simple() {
    // toc-test.pdf has a known structure: 4 top-level + 1 nested = 5 total
    let doc = PDFDocument::open("tests/fixtures/toc-test.pdf").unwrap();
    let chapters = extract_toc(&doc).unwrap();

    assert_eq!(chapters.len(), 5, "Should have 5 total chapters");

    // Check top-level chapters (level 0)
    let top_level: Vec<_> = chapters.iter().filter(|c| c.level == 0).collect();
    assert_eq!(top_level.len(), 4, "Should have 4 top-level chapters");

    // Check nested chapters (level 1)
    let nested: Vec<_> = chapters.iter().filter(|c| c.level == 1).collect();
    assert_eq!(nested.len(), 1, "Should have 1 nested chapter");

    // Verify first chapter
    assert_eq!(chapters[0].title, "Chapter 1: Introduction");
    assert_eq!(chapters[0].level, 0);
    assert_eq!(chapters[0].page_number, 0); // Page numbers are 0-indexed in mupdf
}
