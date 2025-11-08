use voxpdf_core::PDFDocument;
use lopdf::content::Content;

fn main() {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();

    // Get page
    let pages = doc.doc.get_pages();
    let page_id = pages.get(&1).unwrap();

    // Get page dict
    let page_obj = doc.doc.get_object(*page_id).unwrap();
    let page_dict = page_obj.as_dict().unwrap();

    // Get contents
    let contents_ref = page_dict.get(b"Contents").unwrap();

    // Manually decode (using the custom decoder)
    use voxpdf_core::extraction::content_stream::decode_content_stream;
    let content_text = decode_content_stream(&doc.doc, contents_ref).unwrap();

    println!("Content length: {} bytes", content_text.len());
    println!("\nRaw content (first 500 chars):\n{}\n", &content_text[..content_text.len().min(500)]);

    // Parse operations
    let content = Content::decode(content_text.as_bytes()).unwrap();

    println!("\n=== Parsed {} operations ===", content.operations.len());
    for (i, op) in content.operations.iter().enumerate() {
        println!("{}: {:?}", i, op);
    }
}
