use voxpdf_core::pdf::PDFDocument;

#[test]
fn test_open_simple_pdf() {
    let path = "tests/fixtures/simple.pdf";
    let doc = PDFDocument::open(path).expect("Failed to open PDF");
    assert_eq!(doc.page_count(), 1);
}

#[test]
fn test_open_nonexistent_pdf() {
    let path = "tests/fixtures/nonexistent.pdf";
    let result = PDFDocument::open(path);
    assert!(result.is_err());
}
