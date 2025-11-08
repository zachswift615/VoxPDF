use crate::error::{Result, VoxPDFError};
use crate::pdf::PDFDocument;
use super::content_stream::decode_content_stream;

pub fn extract_page_text(doc: &PDFDocument, page_num: u32) -> Result<String> {
    let pages = doc.doc.get_pages();
    let page_number = page_num + 1; // Convert from 0-based to 1-based

    // Verify the page exists
    if !pages.contains_key(&page_number) {
        return Err(VoxPDFError::PageNotFound(page_num, pages.len()));
    }

    // Try lopdf's built-in extract_text first
    let text = doc.doc.extract_text(&[page_number])?;

    // If extract_text returns empty, fall back to manual parsing
    if text.is_empty() {
        return extract_text_from_content_stream(doc, page_num);
    }

    Ok(text)
}

/// Extract text by parsing the PDF content stream directly.
/// This is a fallback for when lopdf's extract_text() doesn't work.
fn extract_text_from_content_stream(doc: &PDFDocument, page_num: u32) -> Result<String> {
    let pages = doc.doc.get_pages();
    let page_id = pages
        .get(&(page_num + 1))
        .ok_or(VoxPDFError::PageNotFound(page_num, pages.len()))?;

    // Get page dictionary
    let page_obj = doc.doc.get_object(*page_id)?;
    let page_dict = page_obj.as_dict()
        .map_err(|_| VoxPDFError::ExtractionError("Page is not a dictionary".to_string()))?;

    // Get contents reference
    let contents_ref = page_dict.get(b"Contents")
        .map_err(|_| VoxPDFError::ExtractionError("No Contents in page".to_string()))?;

    // Decode content stream (using the same approach as words.rs)
    let content_text = decode_content_stream(&doc.doc, contents_ref)?;

    // Extract just the text (without positions)
    let text = extract_text_from_operators(&content_text)?;

    Ok(text)
}

/// Extract text strings from PDF operators.
/// Parses operators like: (Hello) Tj
fn extract_text_from_operators(content: &str) -> Result<String> {
    let mut text_parts = Vec::new();
    let tokens: Vec<&str> = content.split_whitespace().collect();
    let mut i = 0;

    while i < tokens.len() {
        let token = tokens[i];

        if token == "Tj" || token == "TJ" {
            // Show text: (string) Tj or [(strings)] TJ
            if i >= 1 {
                let text_token = tokens[i - 1];
                if let Some(text) = extract_pdf_string(text_token) {
                    text_parts.push(text);
                }
            }
        }

        i += 1;
    }

    Ok(text_parts.join(" "))
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
    fn test_text_extraction_unit() {
        // Unit tests if needed
    }
}
