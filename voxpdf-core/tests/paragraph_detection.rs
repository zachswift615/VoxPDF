use voxpdf_core::extraction::{detect_paragraphs, extract_word_positions};
use voxpdf_core::PDFDocument;

#[test]
fn test_single_paragraph() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();
    let paragraphs = detect_paragraphs(words);

    // Simple PDF should have 1 paragraph
    assert_eq!(paragraphs.len(), 1);
    assert!(paragraphs[0].text.contains("Hello"));
    assert!(paragraphs[0].text.contains("World"));
}

// Will add more tests with multi-paragraph PDFs later
