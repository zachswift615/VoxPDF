use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_word_positions;

#[test]
fn test_extract_word_positions_simple() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    // Should have at least 2 words
    assert!(words.len() >= 2);

    // All words should have valid bounds
    for word in &words {
        assert!(word.bounds.width > 0.0);
        assert!(word.bounds.height > 0.0);
        assert!(!word.text.is_empty());
        assert_eq!(word.page_number, 0);
    }
}

#[test]
fn test_word_positions_accuracy() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    let hello = words.iter().find(|w| w.text.contains("Hello")).unwrap();
    let world = words.iter().find(|w| w.text.contains("World")).unwrap();

    // Words should be on same line (similar Y)
    assert!((hello.bounds.y - world.bounds.y).abs() < 5.0);

    // "World" should be to the right of "Hello"
    assert!(world.bounds.x > hello.bounds.x);
}
