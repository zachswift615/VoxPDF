use voxpdf_core::extraction::extract_word_positions;
use voxpdf_core::PDFDocument;

fn main() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").expect("Failed to open simple.pdf");

    let words = extract_word_positions(&doc, 0).expect("Failed to extract word positions");

    println!("Extracted {} words:", words.len());
    for (i, word) in words.iter().enumerate() {
        println!(
            "  {}: '{}' at ({}, {}) size={}x{}",
            i, word.text, word.bounds.x, word.bounds.y, word.bounds.width, word.bounds.height
        );
    }
}
