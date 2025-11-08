//! Error types for VoxPDF operations.
//!
//! This module defines the [`VoxPDFError`] enum which represents all errors
//! that can occur during PDF processing, including I/O errors, invalid PDF
//! files, and page access errors.

use thiserror::Error;

/// Error type for all VoxPDF operations.
///
/// This enum wraps underlying errors from lopdf and I/O operations,
/// providing consistent error handling across the VoxPDF library.
#[derive(Debug, Error)]
pub enum VoxPDFError {
    /// PDF file could not be opened or parsed.
    ///
    /// This typically indicates a corrupted or invalid PDF file.
    #[error("Failed to open PDF: {0}")]
    InvalidPDF(String),

    /// Requested page does not exist in the document.
    ///
    /// Contains the requested page number and the actual page count.
    #[error("Page {0} not found (document has {1} pages)")]
    PageNotFound(u32, usize),

    /// I/O error occurred during file operations.
    ///
    /// Automatically converted from `std::io::Error`.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Error from the underlying lopdf library.
    ///
    /// Automatically converted from `lopdf::Error`.
    #[error("lopdf error: {0}")]
    Lopdf(#[from] lopdf::Error),
}

/// Type alias for Results using [`VoxPDFError`].
///
/// This is the standard Result type used throughout VoxPDF.
pub type Result<T> = std::result::Result<T, VoxPDFError>;
