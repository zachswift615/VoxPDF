use voxpdf_core::PDFDocument;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/ai-agents-book.pdf"
    };

    println!("Opening PDF: {}", path);
    let doc = PDFDocument::open(path).expect("Failed to open PDF");
    println!("✅ PDF opened successfully");
    println!("   Pages: {}", doc.page_count());

    // Try to get outlines directly with full error details
    println!("\nAttempting to extract outlines...");
    match doc._debug_outlines() {
        Ok(outlines) => {
            println!("✅ Success! Got {} top-level outline entries", outlines.len());

            if !outlines.is_empty() {
                println!("\nFirst few entries:");
                for (i, outline) in outlines.iter().take(5).enumerate() {
                    println!("  [{}] {} (page: {:?})", i, outline.title, outline.page);
                    if !outline.down.is_empty() {
                        println!("      └─ {} children", outline.down.len());
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to extract outlines!");
            println!("   Error: {:?}", e);
            println!("   Display: {}", e);
        }
    }
}
