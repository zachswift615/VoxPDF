use lopdf::{Document, Object};
use flate2::read::ZlibDecoder;
use std::io::Read;

fn main() {
    let doc = Document::load("tests/fixtures/simple.pdf").expect("Failed to load PDF");

    let pages = doc.get_pages();
    let page_id = pages.get(&1).expect("No page 1");

    // Get the page dictionary
    let page_obj = doc.get_object(*page_id).expect("Failed to get page");
    let page_dict = page_obj.as_dict().expect("Page is not a dict");

    // Get contents reference
    let contents_ref = page_dict.get(b"Contents").expect("No Contents");

    if let Object::Reference(ref_id) = contents_ref {
        let stream_obj = doc.get_object(*ref_id).expect("Failed to get stream");

        if let Ok(stream) = stream_obj.as_stream() {
            println!("=== Stream Dictionary ===");
            println!("{:?}", stream.dict);

            println!("\n=== Raw Stream Data (first 200 bytes) ===");
            let raw_data = &stream.content;
            println!("{:?}", &raw_data[..raw_data.len().min(200)]);
            println!("Raw data length: {}", raw_data.len());

            // Try to manually decode ASCII85
            println!("\n=== Manual ASCII85 Decode ===");
            let ascii85_str = String::from_utf8_lossy(raw_data);
            println!("ASCII85 string (len={}): {}", ascii85_str.len(), ascii85_str);

            // Check if it already has the end marker
            let to_decode = if ascii85_str.contains("~>") {
                // Add start marker
                format!("<~{}", ascii85_str)
            } else {
                // Add both markers
                format!("<~{}~>", ascii85_str)
            };

            println!("Decoding: {}", to_decode);

            match ascii85::decode(&to_decode) {
                Ok(decoded_ascii85) => {
                    println!("ASCII85 decoded {} bytes", decoded_ascii85.len());
                    println!("First 100 bytes: {:?}", &decoded_ascii85[..decoded_ascii85.len().min(100)]);

                    // Now try Flate decompression
                    println!("\n=== Flate Decompression ===");
                    let mut decoder = ZlibDecoder::new(&decoded_ascii85[..]);
                    let mut decompressed = Vec::new();
                    match decoder.read_to_end(&mut decompressed) {
                        Ok(_) => {
                            println!("Decompressed {} bytes", decompressed.len());
                            println!("Content: {}", String::from_utf8_lossy(&decompressed));
                        }
                        Err(e) => {
                            println!("Flate decompression error: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("ASCII85 decode error: {:?}", e);
                }
            }
        }
    }
}
