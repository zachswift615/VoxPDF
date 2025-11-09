use voxpdf_core::extraction::extract_toc;
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

    // Extract TOC using the extraction function
    match extract_toc(&doc) {
        Ok(toc_entries) => {
            println!("\n✅ TOC extracted: {} entries total", toc_entries.len());

            // Group by level
            let mut by_level: std::collections::HashMap<u8, Vec<_>> =
                std::collections::HashMap::new();
            for entry in &toc_entries {
                by_level
                    .entry(entry.level)
                    .or_insert_with(Vec::new)
                    .push(entry);
            }

            println!("\nEntries by level:");
            for level in 0..=3 {
                if let Some(entries) = by_level.get(&level) {
                    let label = match level {
                        0 => "chapters",
                        1 => "sections",
                        2 => "subsections",
                        _ => "deeper levels",
                    };
                    println!("  Level {}: {} {}", level, entries.len(), label);
                }
            }

            println!("\nFirst 10 entries:");
            for (i, entry) in toc_entries.iter().take(10).enumerate() {
                let indent = "  ".repeat(entry.level as usize);
                println!(
                    "[{}] {}{} (page {}, level {})",
                    i, indent, entry.title, entry.page_number, entry.level
                );
            }

            if toc_entries.len() > 10 {
                println!("... and {} more entries", toc_entries.len() - 10);
            }
        }
        Err(e) => {
            println!("❌ Failed to extract TOC: {:?}", e);
        }
    }
}
