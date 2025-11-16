pub mod hyphenation;
pub mod paragraphs;
pub mod parallel;
pub mod streaming;
pub mod text;
pub mod toc;
pub mod words;

pub use hyphenation::reassemble_hyphenated_words;
pub use paragraphs::detect_paragraphs;
pub use parallel::{extract_pages_parallel, PageExtractionResult};
pub use streaming::{StreamingExtractor, ExtractionEvent};
pub use text::extract_page_text;
pub use toc::extract_toc;
pub use words::extract_word_positions;
