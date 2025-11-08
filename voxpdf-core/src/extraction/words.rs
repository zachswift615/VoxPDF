use crate::error::{Result, VoxPDFError};
use crate::models::{Rect, Word};
use crate::pdf::PDFDocument;
use lopdf::Object;
use flate2::read::ZlibDecoder;
use std::io::Read;

/// Extract word positions from a PDF page.
///
/// This is a SPIKE implementation to validate lopdf's capabilities for
/// extracting word-level position information.
///
/// # Arguments
///
/// * `doc` - The PDF document
/// * `page_num` - Zero-indexed page number
///
/// # Returns
///
/// A vector of words with their bounding boxes, or an error if extraction fails.
pub fn extract_word_positions(doc: &PDFDocument, page_num: u32) -> Result<Vec<Word>> {
    // Get the page using lopdf's API
    let pages = doc.doc.get_pages();
    let page_id = pages
        .get(&(page_num + 1))
        .ok_or_else(|| VoxPDFError::ExtractionError(format!("Page {} not found", page_num)))?;

    // Get page dictionary
    let page_obj = doc.doc.get_object(*page_id)?;
    let page_dict = page_obj.as_dict()
        .map_err(|_| VoxPDFError::ExtractionError("Page is not a dictionary".to_string()))?;

    // Get contents reference
    let contents_ref = page_dict.get(b"Contents")
        .map_err(|_| VoxPDFError::ExtractionError("No Contents in page".to_string()))?;

    // Decode the content stream manually (workaround for lopdf ASCII85+Flate bug)
    let content_text = decode_content_stream(&doc.doc, contents_ref)?;

    // Parse the content text to extract words
    let words = parse_content_text(&content_text, page_num)?;

    Ok(words)
}

/// Manually decode a PDF content stream
///
/// This is a workaround for lopdf 0.32's bug with ASCII85+Flate filter combinations.
/// lopdf's get_and_decode_page_content() fails to properly decode streams with
/// Filter: [/ASCII85Decode /FlateDecode]
fn decode_content_stream(doc: &lopdf::Document, contents: &Object) -> Result<String> {
    match contents {
        Object::Reference(ref_id) => {
            let stream_obj = doc.get_object(*ref_id)?;
            if let Ok(stream) = stream_obj.as_stream() {
                let raw_data = &stream.content;

                // Check the filter type
                if let Ok(filter) = stream.dict.get(b"Filter") {
                    match filter {
                        Object::Array(filters) => {
                            // Check if it's ASCII85 + Flate
                            let has_ascii85 = filters.iter().any(|f| {
                                if let Ok(name) = f.as_name_str() {
                                    name == "ASCII85Decode"
                                } else {
                                    false
                                }
                            });

                            let has_flate = filters.iter().any(|f| {
                                if let Ok(name) = f.as_name_str() {
                                    name == "FlateDecode"
                                } else {
                                    false
                                }
                            });

                            if has_ascii85 && has_flate {
                                // Manual decode: ASCII85 -> Flate
                                let ascii85_str = String::from_utf8_lossy(raw_data);

                                // Add start marker if needed
                                let to_decode = if ascii85_str.contains("~>") {
                                    format!("<~{}", ascii85_str)
                                } else {
                                    format!("<~{}~>", ascii85_str)
                                };

                                let decoded_ascii85 = ascii85::decode(&to_decode)
                                    .map_err(|e| VoxPDFError::ExtractionError(format!("ASCII85 decode error: {:?}", e)))?;

                                let mut decoder = ZlibDecoder::new(&decoded_ascii85[..]);
                                let mut decompressed = Vec::new();
                                decoder.read_to_end(&mut decompressed)
                                    .map_err(|e| VoxPDFError::ExtractionError(format!("Flate decode error: {:?}", e)))?;

                                return Ok(String::from_utf8_lossy(&decompressed).to_string());
                            }
                        }
                        _ => {}
                    }
                }

                // Fall back to lopdf's decompression
                match stream.decompressed_content() {
                    Ok(data) => Ok(String::from_utf8_lossy(&data).to_string()),
                    Err(e) => Err(VoxPDFError::ExtractionError(format!("Failed to decompress stream: {:?}", e)))
                }
            } else {
                Err(VoxPDFError::ExtractionError("Content is not a stream".to_string()))
            }
        }
        _ => Err(VoxPDFError::ExtractionError("Unexpected Contents type".to_string()))
    }
}

/// Parse PDF content stream text to extract words with positions.
///
/// Expected format from ReportLab:
/// BT 1 0 0 1 100 592 Tm (Hello) Tj T* ET
/// BT 1 0 0 1 160 592 Tm (World) Tj T* ET
///
/// Operators:
/// - BT/ET: Begin/End text object
/// - Tm: Set text matrix - format: a b c d e f Tm (e=x, f=y)
/// - Tj: Show text - format: (text) Tj
/// - Tf: Set font - format: /FontName size Tf
fn parse_content_text(content: &str, page_num: u32) -> Result<Vec<Word>> {
    let mut words = Vec::new();
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut font_size = 12.0;

    // Split into tokens
    let tokens: Vec<&str> = content.split_whitespace().collect();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i];

        match token {
            "Tf" => {
                // Set font: /FontName size Tf
                if i >= 1 {
                    if let Ok(size) = tokens[i - 1].parse::<f32>() {
                        font_size = size;
                    }
                }
            }
            "Tm" => {
                // Set text matrix: a b c d e f Tm
                // e (i-2) is x, f (i-1) is y
                if i >= 6 {
                    if let (Ok(x), Ok(y)) = (
                        tokens[i - 2].parse::<f32>(),
                        tokens[i - 1].parse::<f32>(),
                    ) {
                        current_x = x;
                        current_y = y;
                    }
                }
            }
            "Tj" => {
                // Show text: (string) Tj
                if i >= 1 {
                    let text_token = tokens[i - 1];
                    if let Some(text) = extract_pdf_string(text_token) {
                        // Estimate width (rough approximation for Helvetica)
                        let width = text.len() as f32 * font_size * 0.5;

                        words.push(Word::new(
                            text,
                            Rect::new(current_x, current_y, width, font_size),
                            page_num,
                        ));
                    }
                }
            }
            _ => {}
        }

        i += 1;
    }

    Ok(words)
}

/// Extract text from a PDF string literal: (text) -> text
fn extract_pdf_string(s: &str) -> Option<String> {
    if s.starts_with('(') && s.ends_with(')') {
        Some(s[1..s.len() - 1].to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pdf_string() {
        assert_eq!(extract_pdf_string("(Hello)"), Some("Hello".to_string()));
        assert_eq!(extract_pdf_string("(Hello World)"), Some("Hello World".to_string()));
        assert_eq!(extract_pdf_string("Hello"), None);
    }
}
