use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_toc;

#[test]
#[ignore] // Ignore until we have toc.pdf fixture
fn test_extract_toc() {
    let doc = PDFDocument::open("tests/fixtures/toc.pdf").unwrap();
    let chapters = extract_toc(&doc).unwrap();

    assert!(!chapters.is_empty());
    assert_eq!(chapters[0].level, 0); // Top-level chapter
}

#[test]
fn test_extract_toc_empty() {
    // simple.pdf has no outline
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let chapters = extract_toc(&doc).unwrap();

    assert_eq!(chapters.len(), 0);
}
