use crate::models::{Word, Paragraph};

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
        if (word.bounds.y - current_y).abs() < Y_THRESHOLD {
            // Same line
            current_line.push(word);
        } else {
            // New line
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_line = vec![word.clone()];
            current_y = word.bounds.y;
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
    let mut prev_line_y: Option<f32> = None;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let line_y = line[0].bounds.y;
        let line_height = line[0].bounds.height;

        match prev_line_y {
            None => {
                // First line
                current_para_lines.push(line.clone());
                prev_line_y = Some(line_y);
            }
            Some(prev_y) => {
                let spacing = (line_y - prev_y).abs();

                // If spacing > 2x line height, start new paragraph
                if spacing > line_height * 2.0 {
                    // Finish current paragraph
                    paragraphs.push(create_paragraph_from_lines(
                        paragraphs.len(),
                        current_para_lines
                    ));
                    current_para_lines = vec![line.clone()];
                } else {
                    // Continue current paragraph
                    current_para_lines.push(line.clone());
                }

                prev_line_y = Some(line_y);
            }
        }
    }

    // Add final paragraph
    if !current_para_lines.is_empty() {
        paragraphs.push(create_paragraph_from_lines(
            paragraphs.len(),
            current_para_lines
        ));
    }

    paragraphs
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
            Word::new("Hello", Rect::new(10.0, 100.0, 30.0, 12.0), 0),
            Word::new("World", Rect::new(50.0, 100.0, 30.0, 12.0), 0),
        ];

        let lines = group_words_into_lines(words);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].len(), 2);
    }

    #[test]
    fn test_group_words_different_lines() {
        let words = vec![
            Word::new("Line1", Rect::new(10.0, 100.0, 30.0, 12.0), 0),
            Word::new("Line2", Rect::new(10.0, 120.0, 30.0, 12.0), 0),
        ];

        let lines = group_words_into_lines(words);
        assert_eq!(lines.len(), 2);
    }
}
