use std::env;
use voxpdf_core::{
    extraction::{detect_paragraphs, extract_word_positions},
    PDFDocument,
};

fn main() {
    println!("=== VoxPDF Paragraph Detection Demo ===\n");

    // Get PDF path from args or use default
    let args: Vec<String> = env::args().collect();
    let pdf_path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/simple.pdf"
    };

    // Open the PDF
    let doc = PDFDocument::open(pdf_path).expect("Failed to open PDF");

    println!("PDF: {}", pdf_path);
    println!("Pages: {}\n", doc.page_count());

    // Extract words from first page
    let words = extract_word_positions(&doc, 0).expect("Failed to extract words");

    println!("Extracted {} words from page 0:", words.len());
    for word in &words {
        println!(
            "  - '{}' at ({:.1}, {:.1})",
            word.text, word.bounds.x, word.bounds.y
        );
    }

    // Detect paragraphs
    let paragraphs = detect_paragraphs(words);

    println!("\n=== Detected {} paragraph(s) ===\n", paragraphs.len());

    for para in &paragraphs {
        println!("Paragraph {}:", para.index);
        println!("  Page: {}", para.page_number);
        println!("  Words: {}", para.word_count());
        println!("  Text: \"{}\"\n", para.text);
    }

    println!("✅ Demo complete!");
}
