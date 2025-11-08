use lopdf::{content::Content, Document};

fn main() {
    let doc = Document::load("tests/fixtures/simple.pdf").expect("Failed to load PDF");

    println!("=== PDF Document Info ===");
    println!("Version: {:?}", doc.version);

    let pages = doc.get_pages();
    println!("\nPages: {:?}", pages);

    // Get first page
    if let Some(page_id) = pages.get(&1) {
        println!("\nPage ID: {:?}", page_id);

        if let Ok(page_obj) = doc.get_object(*page_id) {
            println!("\nPage Object:");
            println!("{:#?}", page_obj);

            if let Ok(page_dict) = page_obj.as_dict() {
                if let Ok(contents) = page_dict.get(b"Contents") {
                    println!("\n=== Contents Object ===");
                    println!("{:#?}", contents);

                    // Try using lopdf's Content API
                    match doc.get_and_decode_page_content(*page_id) {
                        Ok(content) => {
                            println!(
                                "\n=== Decoded Page Content (Content object with operations) ==="
                            );
                            for (i, op) in content.operations.iter().enumerate() {
                                println!("Op {}: {:?}", i, op);
                            }
                        }
                        Err(e) => {
                            println!("\n=== Error getting page content: {:?} ===", e);
                        }
                    }
                }
            }
        }
    }
}
