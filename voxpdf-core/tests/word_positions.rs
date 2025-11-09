use voxpdf_core::{extraction::extract_word_positions, PDFDocument};

#[test]
fn test_extract_word_positions_simple() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    // Verify words were extracted
    assert!(!words.is_empty(), "Should extract words from simple.pdf");

    // Verify text content (positions may vary, but text should be correct)
    let text: String = words
        .iter()
        .map(|w| w.text.as_str())
        .collect::<Vec<_>>()
        .join(" ");
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
