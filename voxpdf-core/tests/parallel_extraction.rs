use voxpdf_core::{PDFDocument, extraction::extract_pages_parallel};

#[test]
fn test_parallel_extraction() {
    // Using ai-agents-book.pdf which is a multi-page document
    let doc = PDFDocument::open("tests/fixtures/ai-agents-book.pdf").unwrap();
    let page_count = doc.page_count();

    // Test with first 3 pages (or fewer if document is smaller)
    let num_pages = std::cmp::min(3, page_count);
    let pages: Vec<u32> = (0..num_pages).collect();

    let results = extract_pages_parallel(&doc, pages).unwrap();

    assert_eq!(results.len(), num_pages as usize);
    for result in results {
        assert!(!result.words.is_empty(), "Page {} should have words", result.page_num);
        // Note: paragraphs might be empty for some pages (like title pages)
        // so we only check that words were extracted
    }
}

#[test]
fn test_parallel_extraction_single_page() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let pages = vec![0];

    let results = extract_pages_parallel(&doc, pages).unwrap();

    assert_eq!(results.len(), 1);
    assert!(!results[0].words.is_empty());
}

#[test]
fn test_parallel_extraction_preserves_page_order() {
    let doc = PDFDocument::open("tests/fixtures/ai-agents-book.pdf").unwrap();
    let page_count = doc.page_count();

    // Test with first 5 pages (or fewer if document is smaller)
    let num_pages = std::cmp::min(5, page_count);
    let pages: Vec<u32> = (0..num_pages).collect();

    let results = extract_pages_parallel(&doc, pages).unwrap();

    // Verify results are returned in the same order as requested
    for (i, result) in results.iter().enumerate() {
        assert_eq!(result.page_num, i as u32);
    }
}
