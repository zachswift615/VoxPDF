use crate::error::Result;
use lopdf::Document;
use std::path::Path;

pub struct PDFDocument {
    pub doc: Document,
}

impl PDFDocument {
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let doc = Document::load(path)?;
        Ok(Self { doc })
    }

    pub fn page_count(&self) -> usize {
        self.doc.get_pages().len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_page_count() {
        // Unit test for page_count logic
        // Integration tests are in tests/
    }
}
