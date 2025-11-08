//! PDF document loading and management.
//!
//! This module provides the [`PDFDocument`] struct which wraps
//! lopdf's document handling with VoxPDF's error types.

use crate::error::Result;
use lopdf::Document;
use std::path::Path;

/// A PDF document wrapper providing VoxPDF-specific operations.
///
/// This struct wraps lopdf's [`Document`] type and provides a simplified
/// interface for common PDF operations like loading and querying page counts.
///
/// # Examples
///
/// ```no_run
/// use voxpdf_core::pdf::PDFDocument;
///
/// let doc = PDFDocument::open("example.pdf")?;
/// println!("Document has {} pages", doc.page_count());
/// # Ok::<(), voxpdf_core::error::VoxPDFError>(())
/// ```
pub struct PDFDocument {
    /// The underlying lopdf Document.
    ///
    /// Exposed publicly to allow direct access to lopdf functionality
    /// when needed.
    pub doc: Document,
}

impl PDFDocument {
    /// Opens a PDF document from a file path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the PDF file to open
    ///
    /// # Returns
    ///
    /// Returns `Ok(PDFDocument)` if the file was successfully loaded,
    /// or `Err(VoxPDFError)` if the file could not be opened or parsed.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use voxpdf_core::pdf::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// # Ok::<(), voxpdf_core::error::VoxPDFError>(())
    /// ```
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let doc = Document::load(path)?;
        Ok(Self { doc })
    }

    /// Returns the number of pages in the document.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use voxpdf_core::pdf::PDFDocument;
    ///
    /// let doc = PDFDocument::open("example.pdf")?;
    /// let count = doc.page_count();
    /// println!("Document has {} pages", count);
    /// # Ok::<(), voxpdf_core::error::VoxPDFError>(())
    /// ```
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
