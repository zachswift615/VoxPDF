use voxpdf_core::{PDFDocument, extraction::{extract_word_positions, detect_paragraphs}};
use std::env;

fn main() {
    println!("=== VoxPDF Paragraph Detection Demo ===\n");

    let args: Vec<String> = env::args().collect();
    let pdf_path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/simple.pdf"
    };

    let page_num = if args.len() > 2 {
        args[2].parse::<u32>().unwrap_or(0)
    } else {
        1  // Default to page 1 (not cover)
    };

    let doc = PDFDocument::open(pdf_path)
        .expect("Failed to open PDF");

    println!("PDF: {}", pdf_path);
    println!("Total Pages: {}", doc.page_count());
    println!("Extracting from page: {}\n", page_num);

    let words = extract_word_positions(&doc, page_num)
        .expect("Failed to extract words");

    println!("Extracted {} words\n", words.len());

    // Show first 10 words
    println!("First 10 words:");
    for word in words.iter().take(10) {
        println!("  '{}' at ({:.1}, {:.1})", word.text, word.bounds.x, word.bounds.y);
    }

    let paragraphs = detect_paragraphs(words);

    println!("\n=== Detected {} paragraph(s) ===\n", paragraphs.len());

    // Show first 3 paragraphs
    for para in paragraphs.iter().take(3) {
        println!("Paragraph {}:", para.index);
        println!("  Words: {}", para.word_count());
        println!("  Text: \"{}\"\n",
            if para.text.len() > 200 {
                format!("{}...", &para.text[..200])
            } else {
                para.text.clone()
            }
        );
    }

    println!("✅ Demo complete!");
}
