use crate::models::{Paragraph, Word};

pub fn detect_paragraphs(words: Vec<Word>) -> Vec<Paragraph> {
    if words.is_empty() {
        return Vec::new();
    }

    // Algorithm:
    // 1. Group words into lines by Y-position
    // 2. Merge lines into paragraphs by spacing
    // 3. Break on large vertical gaps (>2x line height)

    let lines = group_words_into_lines(words);
    merge_lines_into_paragraphs(lines)
}

fn group_words_into_lines(words: Vec<Word>) -> Vec<Vec<Word>> {
    if words.is_empty() {
        return Vec::new();
    }

    let mut lines: Vec<Vec<Word>> = Vec::new();
    let mut current_line: Vec<Word> = Vec::new();
    let mut current_y = words[0].bounds.y;

    const Y_THRESHOLD: f32 = 5.0; // Words within 5pts are on same line

    for word in words {
        let word_y = word.bounds.y;
        if (word_y - current_y).abs() < Y_THRESHOLD {
            // Same line
            current_line.push(word);
        } else {
            // New line
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_y = word_y;
            current_line = vec![word];
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}

fn merge_lines_into_paragraphs(lines: Vec<Vec<Word>>) -> Vec<Paragraph> {
    if lines.is_empty() {
        return Vec::new();
    }

    let mut paragraphs: Vec<Paragraph> = Vec::new();
    let mut current_para_lines: Vec<Vec<Word>> = Vec::new();
    let mut prev_line: Option<&Vec<Word>> = None;

    for line in &lines {
        if line.is_empty() {
            continue;
        }

        match prev_line {
            None => {
                // First line
                current_para_lines.push(line.clone());
                prev_line = Some(line);
            }
            Some(prev) => {
                if should_break_paragraph(prev, line) {
                    // Finish current paragraph
                    paragraphs.push(create_paragraph_from_lines(
                        paragraphs.len(),
                        current_para_lines,
                    ));
                    current_para_lines = vec![line.clone()];
                } else {
                    // Continue current paragraph
                    current_para_lines.push(line.clone());
                }

                prev_line = Some(line);
            }
        }
    }

    // Add final paragraph
    if !current_para_lines.is_empty() {
        paragraphs.push(create_paragraph_from_lines(
            paragraphs.len(),
            current_para_lines,
        ));
    }

    paragraphs
}

/// Determine if we should start a new paragraph based on multiple heuristics
fn should_break_paragraph(prev_line: &[Word], current_line: &[Word]) -> bool {
    if prev_line.is_empty() || current_line.is_empty() {
        return false;
    }

    let prev_y = prev_line[0].bounds.y;
    let prev_height = prev_line[0].bounds.height;
    let prev_font_size = prev_line[0].font_size;
    let prev_x = prev_line[0].bounds.x;

    let current_y = current_line[0].bounds.y;
    let current_font_size = current_line[0].font_size;
    let current_x = current_line[0].bounds.x;

    let spacing = (current_y - prev_y).abs();

    // Heuristic 1: Large vertical spacing (> 2x line height)
    if spacing > prev_height * 2.0 {
        return true;
    }

    // Heuristic 2: Font size increase (likely a heading)
    // If current line has significantly larger font (>15% increase), it's likely a heading
    if current_font_size > prev_font_size * 1.15 {
        return true;
    }

    // Heuristic 3: Font size decrease after larger font (end of heading)
    // If previous line had larger font and current line is smaller, break paragraph
    if prev_font_size > current_font_size * 1.15 {
        return true;
    }

    // Heuristic 4: Significant indentation change combined with spacing
    // If indentation changes significantly (> 10pt) AND there's moderate spacing (> 1.3x height)
    let indent_change = (current_x - prev_x).abs();
    if indent_change > 10.0 && spacing > prev_height * 1.3 {
        return true;
    }

    // Heuristic 5: Line length heuristic for headings
    // Short lines (< 60% of typical line width) with larger font are likely headings
    let prev_line_width: f32 = prev_line.iter().map(|w| w.bounds.width).sum();
    let current_line_width: f32 = current_line.iter().map(|w| w.bounds.width).sum();
    let avg_line_width = (prev_line_width + current_line_width) / 2.0;

    // If previous line is short and next line is normal length with moderate spacing
    if prev_line_width < avg_line_width * 0.6 && spacing > prev_height * 1.2 {
        return true;
    }

    false
}

fn create_paragraph_from_lines(index: usize, lines: Vec<Vec<Word>>) -> Paragraph {
    let mut all_words: Vec<Word> = Vec::new();
    let mut text_parts: Vec<String> = Vec::new();

    for line in lines {
        let line_text: Vec<String> = line.iter().map(|w| w.text.clone()).collect();
        text_parts.push(line_text.join(" "));
        all_words.extend(line);
    }

    let text = text_parts.join(" ");
    let page_number = all_words.first().map(|w| w.page_number).unwrap_or(0);

    Paragraph::new(index, text, page_number, all_words)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Rect;

    #[test]
    fn test_group_words_same_line() {
        let words = vec![
            Word::new("Hello", Rect::new(10.0, 100.0, 30.0, 12.0), 0, 12.0),
            Word::new("World", Rect::new(50.0, 100.0, 30.0, 12.0), 0, 12.0),
        ];

        let lines = group_words_into_lines(words);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 2);
    }

    #[test]
    fn test_group_words_different_lines() {
        let words = vec![
            Word::new("Line1", Rect::new(10.0, 100.0, 30.0, 12.0), 0, 12.0),
            Word::new("Line2", Rect::new(10.0, 120.0, 30.0, 12.0), 0, 12.0),
        ];

        let lines = group_words_into_lines(words);
        assert_eq!(lines.len(), 2);
    }
}
