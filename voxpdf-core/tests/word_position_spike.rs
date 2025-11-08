use voxpdf_core::{PDFDocument, Word};
use voxpdf_core::extraction::extract_word_positions;

#[test]
fn spike_validates_word_positions() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf")
        .expect("Failed to open simple.pdf");

    let words = extract_word_positions(&doc, 0)
        .expect("Failed to extract word positions");

    // Find "Hello" and "World"
    let hello = words.iter().find(|w| w.text == "Hello")
        .expect("Should find 'Hello'");
    let world = words.iter().find(|w| w.text == "World")
        .expect("Should find 'World'");

    // Validate positions within 10% error
    // Expected positions from simple-positions.json:
    // Hello: x=100.0, y=592.0, width=27.3, height=12.0
    // World: x=160.0, y=592.0, width=31.3, height=12.0

    assert!((hello.bounds.x - 100.0).abs() < 10.0,
            "Hello X position: expected ~100, got {}", hello.bounds.x);
    assert!((hello.bounds.y - 592.0).abs() < 59.2,
            "Hello Y position: expected ~592, got {}", hello.bounds.y);

    assert!((world.bounds.x - 160.0).abs() < 16.0,
            "World X position: expected ~160, got {}", world.bounds.x);
    assert!((world.bounds.y - 592.0).abs() < 59.2,
            "World Y position: expected ~592, got {}", world.bounds.y);

    // Words should have reasonable dimensions
    assert!(hello.bounds.width > 0.0 && hello.bounds.width < 100.0,
            "Hello width: expected >0 and <100, got {}", hello.bounds.width);
    assert!(hello.bounds.height > 0.0 && hello.bounds.height < 30.0,
            "Hello height: expected >0 and <30, got {}", hello.bounds.height);
}
