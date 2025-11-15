use super::Word;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paragraph {
    pub index: usize,
    pub text: String,
    pub page_number: u32,
    pub words: Vec<Word>,
}

impl Paragraph {
    pub fn new(index: usize, text: String, page_number: u32, words: Vec<Word>) -> Self {
        Self {
            index,
            text,
            page_number,
            words,
        }
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Rect;

    #[test]
    fn test_paragraph_creation() {
        let word = Word::new("test", Rect::new(0.0, 0.0, 10.0, 10.0), 0, 12.0);

        let para = Paragraph::new(0, "test".to_string(), 0, vec![word]);

        assert_eq!(para.index, 0);
        assert_eq!(para.text, "test");
        assert_eq!(para.word_count(), 1);
    }
}
