use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_word_positions;

fn main() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let words = extract_word_positions(&doc, 0).unwrap();

    println!("Extracted {} words:", words.len());
    for word in &words {
        println!("  '{}' at ({:.1}, {:.1}) size {:.1}x{:.1}",
            word.text, word.bounds.x, word.bounds.y, word.bounds.width, word.bounds.height);
    }
}
