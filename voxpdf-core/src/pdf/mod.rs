use crate::error::{Result, VoxPDFError};
use mupdf::Document as MuPdfDocument;

/// Wrapper around MuPDF document
pub struct PDFDocument {
    pub(crate) doc: MuPdfDocument,
    path: String,
}

impl PDFDocument {
    /// Open a PDF file from disk
    ///
    /// # Example
    /// ```no_run
    /// use voxpdf_core::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn open(path: &str) -> Result<Self> {
        let doc = MuPdfDocument::open(path)
            .map_err(|e| VoxPDFError::InvalidPDF(format!("Failed to open PDF: {}", e)))?;

        Ok(PDFDocument {
            doc,
            path: path.to_string(),
        })
    }

    /// Get the number of pages in the document
    ///
    /// # Example
    /// ```no_run
    /// use voxpdf_core::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// let count = doc.page_count();
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn page_count(&self) -> u32 {
        self.doc.page_count().unwrap_or(0) as u32
    }

    /// Get the file path of this document
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Try to load outlines and return detailed error info for debugging
    #[doc(hidden)]
    pub fn _debug_outlines(&self) -> std::result::Result<Vec<mupdf::Outline>, mupdf::Error> {
        self.doc.outlines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_count() {
        let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();
        assert_eq!(doc.page_count(), 1);
    }
}
