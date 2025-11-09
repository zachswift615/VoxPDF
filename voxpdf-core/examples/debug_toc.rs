use voxpdf_core::PDFDocument;
use voxpdf_core::extraction::extract_toc;

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

    // Extract TOC using the extraction function
    match extract_toc(&doc) {
        Ok(chapters) => {
            println!("\n✅ TOC extracted: {} chapters total", chapters.len());

            // Group by level
            let mut by_level: std::collections::HashMap<u8, Vec<_>> = std::collections::HashMap::new();
            for chapter in &chapters {
                by_level.entry(chapter.level).or_insert_with(Vec::new).push(chapter);
            }

            println!("\nChapters by level:");
            for level in 0..=3 {
                if let Some(chaps) = by_level.get(&level) {
                    println!("  Level {}: {} chapters", level, chaps.len());
                }
            }

            println!("\nFirst 10 chapters:");
            for (i, chapter) in chapters.iter().take(10).enumerate() {
                let indent = "  ".repeat(chapter.level as usize);
                println!("[{}] {}{} (page {}, level {})",
                    i, indent, chapter.title, chapter.page_number, chapter.level);
            }

            if chapters.len() > 10 {
                println!("... and {} more chapters", chapters.len() - 10);
            }
        }
        Err(e) => {
            println!("❌ Failed to extract TOC: {:?}", e);
        }
    }
}
