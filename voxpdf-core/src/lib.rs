pub mod error;
pub mod extraction;
pub mod ffi;
pub mod models;
pub mod pdf;

pub use error::{Result, VoxPDFError};
pub use models::{Chapter, Paragraph, Rect, Word};
pub use pdf::PDFDocument;
