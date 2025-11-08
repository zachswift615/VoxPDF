use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub level: u8,
    pub page_number: u32,
    pub paragraph_index: usize,
}

impl Chapter {
    pub fn new(title: String, level: u8, page_number: u32, paragraph_index: usize) -> Self {
        Self {
            title,
            level,
            page_number,
            paragraph_index,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chapter_creation() {
        let chapter = Chapter::new("Introduction".to_string(), 0, 1, 0);
        assert_eq!(chapter.title, "Introduction");
        assert_eq!(chapter.level, 0);
        assert_eq!(chapter.page_number, 1);
    }
}
