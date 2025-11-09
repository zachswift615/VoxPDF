use serde::{Deserialize, Serialize};

/// A single entry in the PDF's table of contents (outline/bookmarks).
///
/// Represents any hierarchical entry in the PDF outline structure, including:
/// - Top-level chapters (level 0)
/// - Sections (level 1)
/// - Subsections (level 2)
/// - Deeper nesting levels (level 3+)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TocEntry {
    /// The title/label of this TOC entry
    pub title: String,

    /// Nesting level: 0 = chapter, 1 = section, 2 = subsection, etc.
    pub level: u8,

    /// Page number where this entry points (0-indexed)
    pub page_number: u32,

    /// Index into the paragraph array (for navigation)
    pub paragraph_index: usize,
}

impl TocEntry {
    pub fn new(title: String, level: u8, page_number: u32, paragraph_index: usize) -> Self {
        Self {
            title,
            level,
            page_number,
            paragraph_index,
        }
    }

    /// Returns true if this is a top-level chapter (level 0)
    pub fn is_chapter(&self) -> bool {
        self.level == 0
    }

    /// Returns true if this is a section (level 1)
    pub fn is_section(&self) -> bool {
        self.level == 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toc_entry_creation() {
        let entry = TocEntry::new("Introduction".to_string(), 0, 1, 0);
        assert_eq!(entry.title, "Introduction");
        assert_eq!(entry.level, 0);
        assert_eq!(entry.page_number, 1);
        assert!(entry.is_chapter());
        assert!(!entry.is_section());
    }
}
