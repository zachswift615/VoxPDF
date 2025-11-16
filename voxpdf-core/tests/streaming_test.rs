use voxpdf_core::extraction::{StreamingExtractor, ExtractionEvent};

#[test]
fn test_streaming_extraction() {
    let extractor = StreamingExtractor::new();

    extractor.extract_async("tests/fixtures/simple.pdf".to_string(), 0, 0);

    let mut received_complete = false;
    while let Some(event) = extractor.receive() {
        match event {
            ExtractionEvent::PageComplete { page, paragraphs } => {
                assert_eq!(page, 0);
                assert!(!paragraphs.is_empty());
            }
            ExtractionEvent::Complete => {
                received_complete = true;
                break;
            }
            ExtractionEvent::Error { .. } => panic!("Unexpected error"),
        }
    }

    assert!(received_complete);
}
