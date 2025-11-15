use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Word {
    pub text: String,
    pub bounds: Rect,
    pub page_number: u32,
    pub font_size: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Word {
    pub fn new(text: impl Into<String>, bounds: Rect, page_number: u32, font_size: f32) -> Self {
        Self {
            text: text.into(),
            bounds,
            page_number,
            font_size,
        }
    }
}

impl Rect {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_creation() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        let word = Word::new("test", rect.clone(), 0, 12.0);

        assert_eq!(word.text, "test");
        assert_eq!(word.bounds, rect);
        assert_eq!(word.page_number, 0);
        assert_eq!(word.font_size, 12.0);
    }
}
