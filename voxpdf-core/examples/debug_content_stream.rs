use voxpdf_core::PDFDocument;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pdf_path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/fixtures/simple.pdf"
    };

    let page_num = if args.len() > 2 {
        args[2].parse::<u32>().unwrap_or(0)
    } else {
        0
    };

    println!("Opening: {}", pdf_path);
    println!("Page: {}\n", page_num);

    let doc = PDFDocument::open(pdf_path).expect("Failed to open PDF");

    // Get the page
    let pages = doc.doc.get_pages();
    let page_id = pages.get(&(page_num + 1)).expect("Page not found");

    // Get page object
    let page_obj = doc.doc.get_object(*page_id).expect("Failed to get page");
    let page_dict = page_obj.as_dict().expect("Page is not a dictionary");

    // Get contents
    let contents_ref = page_dict.get(b"Contents").expect("No Contents");

    // Try to decode content stream using lopdf's method
    println!("=== Trying lopdf's decode method ===");
    match doc.doc.get_and_decode_page_content(*page_id) {
        Ok(content) => {
            println!("Number of operations: {}", content.operations.len());

            println!("\n=== All Operations ===");
            for (i, op) in content.operations.iter().take(20).enumerate() {
                println!("  {}: {:?}", i, op);
            }

            // Find text operations
            println!("\n=== Text Operations (Tj, TJ, ') ===");
            let mut count = 0;
            for (i, op) in content.operations.iter().enumerate() {
                match op.operator.as_str() {
                    "Tj" | "TJ" | "'" | "\"" => {
                        println!("  Operation {}: {:?}", i, op);
                        count += 1;
                        if count >= 20 {
                            println!("  ... (showing first 20 text operations)");
                            break;
                        }
                    }
                    _ => {}
                }
            }

            println!("\nTotal text operations: {}", count);
        }
        Err(e) => {
            println!("lopdf decode failed: {:?}", e);
        }
    }
}
