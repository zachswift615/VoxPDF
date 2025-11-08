use lopdf::{Document, Object, ObjectId};

fn main() {
    let doc = Document::load("tests/fixtures/simple.pdf").expect("Failed to load PDF");

    let pages = doc.get_pages();
    let page_id = pages.get(&1).expect("No page 1");

    println!("=== Trying to decode page content manually ===\n");

    // Get page content
    if let Ok(content) = doc.get_and_decode_page_content(*page_id) {
        println!("Content has {} operations", content.operations.len());

        for (i, op) in content.operations.iter().enumerate() {
            println!("\nOp {}: {}", i, op.operator);
            println!("  Operands: {:?}", op.operands);

            // Show string operands as text
            for operand in &op.operands {
                if let Object::String(bytes, _format) = operand {
                    println!("    String: {:?}", String::from_utf8_lossy(bytes));
                }
            }
        }
    } else {
        println!("Failed to get page content");
    }
}
