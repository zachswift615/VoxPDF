use thiserror::Error;

#[derive(Debug, Error)]
pub enum VoxPDFError {
    #[error("Failed to open PDF: {0}")]
    InvalidPDF(String),

    #[error("Page {0} not found (document has {1} pages)")]
    PageNotFound(u32, usize),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("lopdf error: {0}")]
    Lopdf(#[from] lopdf::Error),
}

pub type Result<T> = std::result::Result<T, VoxPDFError>;
