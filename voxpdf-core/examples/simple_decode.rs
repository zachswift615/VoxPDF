use lopdf::Document;

fn main() {
    let doc = Document::load("tests/fixtures/simple.pdf").expect("Failed to load PDF");

    // Try extract_text method if available
    let pages = doc.get_pages();
    for (page_num, page_id) in &pages {
        println!("=== Page {} ===", page_num);

        // Try lopdf's built-in text extraction
        match doc.extract_text(&[*page_num]) {
            Ok(text) => {
                println!("Extracted text: {:?}", text);
            }
            Err(e) => {
                println!("Error extracting text: {:?}", e);
            }
        }
    }
}
