/// Quick test to see if lopdf can extract TOC from AI Agents book
/// where mupdf fails with error code 7
use lopdf::Document;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/ai-agents-book.pdf"
    };

    println!("Testing lopdf TOC extraction on: {}", path);
    println!("================================================\n");

    match Document::load(path) {
        Ok(doc) => {
            println!("✅ PDF loaded successfully");
            println!("   Pages: {}", doc.get_pages().len());

            // Try to get the catalog
            let catalog_dict = doc.catalog().expect("Failed to get catalog");
            println!("✅ Got catalog dictionary");

            // Look for Outlines entry
            match catalog_dict.get(b"Outlines") {
                Ok(outlines_ref) => {
                    println!("✅ Found Outlines entry: {:?}", outlines_ref);

                    // Try to dereference and get the outlines dictionary
                    if let Ok(outlines_id) = outlines_ref.as_reference() {
                        match doc.get_dictionary(outlines_id) {
                            Ok(outlines_dict) => {
                                println!("✅ Got outlines dictionary: {:#?}", outlines_dict);

                                // Look for First entry (first outline item)
                                if let Ok(first_ref) = outlines_dict.get(b"First") {
                                    println!("\n✅ Has First outline entry!");
                                    println!("   Attempting to traverse outline tree...\n");

                                    let mut count = 0;
                                    if let Ok(first_id) = first_ref.as_reference() {
                                        count += traverse_outline(&doc, first_id, 0);
                                    }

                                    println!("\n✅ Total outline entries found: {}", count);
                                } else {
                                    println!("❌ No 'First' entry in outlines");
                                }
                            }
                            Err(e) => {
                                println!("❌ Failed to get outlines dictionary: {:?}", e);
                            }
                        }
                    } else {
                        println!("❌ Outlines is not a reference");
                    }
                }
                Err(_) => {
                    println!("❌ No Outlines entry in catalog");
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to load PDF: {:?}", e);
        }
    }
}

fn traverse_outline(doc: &Document, outline_id: lopdf::ObjectId, level: usize) -> usize {
    let mut count = 0;

    if let Ok(outline_dict) = doc.get_dictionary(outline_id) {
        // Get title
        if let Ok(title_obj) = outline_dict.get(b"Title") {
            if let Ok(title_bytes) = title_obj.as_str() {
                let indent = "  ".repeat(level);
                println!(
                    "{}[{}] {}",
                    indent,
                    count,
                    String::from_utf8_lossy(title_bytes)
                );
                count += 1;
            }
        }

        // Get first child
        if let Ok(first_ref) = outline_dict.get(b"First") {
            if let Ok(first_id) = first_ref.as_reference() {
                count += traverse_outline(doc, first_id, level + 1);
            }
        }

        // Get next sibling
        if let Ok(next_ref) = outline_dict.get(b"Next") {
            if let Ok(next_id) = next_ref.as_reference() {
                count += traverse_outline(doc, next_id, level);
            }
        }
    }

    count
}
