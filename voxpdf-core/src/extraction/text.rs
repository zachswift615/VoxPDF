use crate::error::{Result, VoxPDFError};
use crate::pdf::PDFDocument;
use mupdf::TextPageOptions;

/// Extract plain text from a PDF page
pub fn extract_page_text(doc: &PDFDocument, page_num: u32) -> Result<String> {
    let page = doc.doc.load_page(page_num as i32)
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to load page {}: {}", page_num, e)))?;
    let text_page = page.to_text_page(TextPageOptions::empty())
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to extract text: {}", e)))?;

    let mut text = String::new();

    for block in text_page.blocks() {
        for line in block.lines() {
            for text_char in line.chars() {
                if let Some(c) = text_char.char() {
                    if !c.is_control() {
                        text.push(c);
                    }
                }
            }
            text.push('\n');
        }
        text.push('\n');
    }

    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_extraction_unit() {
        // Integration tests in tests/text_extraction.rs provide coverage
    }
}
