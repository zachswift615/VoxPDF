use crate::error::{Result, VoxPDFError};
use crate::models::{Rect, Word};
use crate::pdf::PDFDocument;
use mupdf::TextPageOptions;

// Pre-allocation capacity estimates for performance optimization
const ESTIMATED_WORDS_PER_PAGE: usize = 500;
const ESTIMATED_CHARS_PER_WORD: usize = 8;

/// Extract word positions from a PDF page using MuPDF.
///
/// Groups consecutive TextChar objects into words based on spacing.
///
/// # Arguments
///
/// * `doc` - The PDF document
/// * `page_num` - Zero-indexed page number
///
/// # Returns
///
/// A vector of words with their bounding boxes.
pub fn extract_word_positions(doc: &PDFDocument, page_num: u32) -> Result<Vec<Word>> {
    // Get the page
    let page = doc.doc.load_page(page_num as i32).map_err(|e| {
        VoxPDFError::ExtractionError(format!("Failed to load page {}: {}", page_num, e))
    })?;

    // Convert to text page
    let text_page = page
        .to_text_page(TextPageOptions::empty())
        .map_err(|e| VoxPDFError::ExtractionError(format!("Failed to extract text: {}", e)))?;

    // Extract characters and group into words
    let mut words = Vec::with_capacity(ESTIMATED_WORDS_PER_PAGE);
    let mut current_word = String::with_capacity(ESTIMATED_CHARS_PER_WORD);
    let mut word_chars: Vec<(char, f32, f32, f32)> = Vec::with_capacity(ESTIMATED_CHARS_PER_WORD); // (char, x, y, size)

    // Word spacing threshold: characters farther apart than this start a new word
    const WORD_SPACING_THRESHOLD: f32 = 3.0;

    for block in text_page.blocks() {
        for line in block.lines() {
            let mut prev_x: Option<f32> = None;

            for text_char in line.chars() {
                let c = match text_char.char() {
                    Some(c) => c,
                    None => continue,
                };

                // Skip control characters and excessive whitespace
                if c.is_control() {
                    continue;
                }

                let origin = text_char.origin();
                let size = text_char.size();
                let quad = text_char.quad();
                let x = origin.x;
                let y = origin.y;
                let char_width = quad.ur.x - quad.ul.x;

                // Check if this character starts a new word
                let is_space = c.is_whitespace();
                let is_new_word = if let Some(prev) = prev_x {
                    let gap = x - prev;
                    gap > WORD_SPACING_THRESHOLD || is_space
                } else {
                    false
                };

                if is_new_word || is_space {
                    // Finish current word
                    if !current_word.trim().is_empty() {
                        let word = create_word_from_chars(
                            current_word.trim().to_string(),
                            &word_chars,
                            page_num,
                        );
                        words.push(word);
                    }
                    current_word.clear();
                    word_chars.clear();

                    // If this is not just whitespace, start new word with this char
                    if !is_space {
                        current_word.push(c);
                        word_chars.push((c, x, y, size));
                    }
                } else {
                    // Continue current word
                    current_word.push(c);
                    word_chars.push((c, x, y, size));
                }

                // Use actual character width from quad instead of approximation
                prev_x = Some(x + char_width);
            }

            // Finish word at end of line
            if !current_word.trim().is_empty() {
                let word =
                    create_word_from_chars(current_word.trim().to_string(), &word_chars, page_num);
                words.push(word);
                current_word.clear();
                word_chars.clear();
            }

            // prev_x is reset implicitly at the start of next line
        }
    }

    Ok(words)
}

/// Create a Word from a collection of characters
fn create_word_from_chars(text: String, chars: &[(char, f32, f32, f32)], page_num: u32) -> Word {
    if chars.is_empty() {
        return Word::new(text, Rect::new(0.0, 0.0, 0.0, 0.0), page_num, 0.0);
    }

    // Calculate bounding box
    let min_x = chars
        .iter()
        .map(|(_, x, _, _)| *x)
        .fold(f32::INFINITY, f32::min);
    let min_y = chars
        .iter()
        .map(|(_, _, y, _)| *y)
        .fold(f32::INFINITY, f32::min);
    let max_x = chars
        .iter()
        .map(|(_, x, _, s)| x + s * 0.6)
        .fold(f32::NEG_INFINITY, f32::max);
    let max_y = chars
        .iter()
        .map(|(_, _, y, s)| y + s)
        .fold(f32::NEG_INFINITY, f32::max);

    let width = max_x - min_x;
    let height = max_y - min_y;

    // Calculate average font size for this word
    let avg_font_size = chars.iter().map(|(_, _, _, s)| s).sum::<f32>() / chars.len() as f32;

    Word::new(text, Rect::new(min_x, min_y, width, height), page_num, avg_font_size)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_operations() {
        // Integration tests in tests/word_positions.rs provide coverage
    }
}
