use voxpdf_core::extraction::extract_page_text;
use voxpdf_core::PDFDocument;

fn main() {
    println!("=== MuPDF Hyphen Fix Demo ===\n");

    // Extract text from simple.pdf
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
    let text = extract_page_text(&doc, 0).unwrap();

    println!("Extracted text:");
    println!("{}", text);
    println!("\n=== Character Analysis ===");

    // Show Unicode codepoints for each character
    for (i, c) in text.chars().enumerate() {
        if !c.is_whitespace() {
            println!(
                "  '{}' -> U+{:04X} ({})",
                c,
                c as u32,
                if c == '\u{2010}' {
                    "HYPHEN ✅"
                } else if c == '!' {
                    "EXCLAMATION ❌"
                } else {
                    "regular char"
                }
            );
        }
    }

    println!("\n✅ With mupdf: Hyphens are proper Unicode U+2010");
    println!("❌ With lopdf: Hyphens were ASCII 0x21 (!) - broken for TTS");
}
