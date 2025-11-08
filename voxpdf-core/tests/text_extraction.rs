use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_page_text;

#[test]
fn test_extract_simple_text() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let text = extract_page_text(&doc, 0).unwrap();

    let expected = include_str!("fixtures/simple.txt");
    assert_eq!(text.trim(), expected.trim());
}

#[test]
fn test_extract_multipage_text() {
    // Will add multi-page.pdf fixture later
    // For now, just test simple.pdf page 0
}
