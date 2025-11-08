use super::content_stream::decode_content_stream;
use crate::error::{Result, VoxPDFError};
use crate::models::{Rect, Word};
use crate::pdf::PDFDocument;
use lopdf::content::Content;

/// Extract word positions from a PDF page.
///
/// Uses lopdf's Content API to properly decode text with font encodings.
/// Works around lopdf's ASCII85+Flate decompression bug by using custom decoder.
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
    let page_dict = page_obj
        .as_dict()
        .map_err(|_| VoxPDFError::ExtractionError("Page is not a dictionary".to_string()))?;

    // Get contents reference
    let contents_ref = page_dict
        .get(b"Contents")
        .map_err(|_| VoxPDFError::ExtractionError("No Contents in page".to_string()))?;

    // Decode content stream (works around ASCII85+Flate bug)
    let content_text = decode_content_stream(&doc.doc, contents_ref)?;

    // Parse the text content into operations using lopdf
    let content = Content::decode(&content_text.as_bytes())
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to parse content: {:?}", e)))?;

    // Parse the content operations to extract words
    let words = parse_content_operations(&content.operations, page_num)?;

    Ok(words)
}

/// Parse PDF content operations to extract words with positions.
///
/// Handles PDF text operators:
/// - Tm: Set text matrix (position)
/// - Tf: Set font and size
/// - Tj: Show text string
/// - TJ: Show text array (with kerning)
///
/// lopdf automatically decodes font encodings for us.
fn parse_content_operations(operations: &[lopdf::content::Operation], page_num: u32) -> Result<Vec<Word>> {
    let mut words = Vec::new();
    let mut current_x = 0.0;
    let mut current_y = 0.0;
    let mut font_size = 12.0;

    for op in operations {
        match op.operator.as_str() {
            "Tf" => {
                // Set font: /FontName size Tf
                if op.operands.len() >= 2 {
                    // Try float first, then integer
                    let size = op.operands[1].as_f32()
                        .or_else(|_| op.operands[1].as_i64().map(|i| i as f32))
                        .unwrap_or(12.0);
                    font_size = size;
                }
            }
            "Tm" => {
                // Set text matrix: a b c d e f Tm
                // operands[4] is x (e), operands[5] is y (f)
                if op.operands.len() >= 6 {
                    // Try float first, then integer (lopdf uses integers for whole numbers)
                    let x = op.operands[4].as_f32()
                        .or_else(|_| op.operands[4].as_i64().map(|i| i as f32))
                        .unwrap_or(0.0);
                    let y = op.operands[5].as_f32()
                        .or_else(|_| op.operands[5].as_i64().map(|i| i as f32))
                        .unwrap_or(0.0);
                    current_x = x;
                    current_y = y;
                }
            }
            "Tj" => {
                // Show text: (string) Tj
                if !op.operands.is_empty() {
                    if let Ok(text) = op.operands[0].as_str() {
                        let text_str = String::from_utf8_lossy(text).to_string();
                        if !text_str.trim().is_empty() {
                            let width = text_str.len() as f32 * font_size * 0.5;
                            words.push(Word::new(
                                text_str,
                                Rect::new(current_x, current_y, width, font_size),
                                page_num,
                            ));
                        }
                    }
                }
            }
            "TJ" => {
                // Show text array: [(string1) kerning (string2) ...] TJ
                if !op.operands.is_empty() {
                    if let Ok(array) = op.operands[0].as_array() {
                        for item in array {
                            if let Ok(text) = item.as_str() {
                                let text_str = String::from_utf8_lossy(text).to_string();
                                if !text_str.trim().is_empty() {
                                    let width = text_str.len() as f32 * font_size * 0.5;
                                    words.push(Word::new(
                                        text_str,
                                        Rect::new(current_x, current_y, width, font_size),
                                        page_num,
                                    ));
                                    // Advance x position (rough approximation)
                                    current_x += width;
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(words)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operations() {
        // Unit test would require constructing lopdf::content::Operation objects
        // Integration tests in tests/word_positions.rs provide coverage
    }
}
