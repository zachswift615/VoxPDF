use rayon::prelude::*;
use crate::error::Result;
use crate::models::{Word, Paragraph};
use crate::pdf::PDFDocument;
use crate::extraction::{extract_word_positions, detect_paragraphs, reassemble_hyphenated_words};

pub struct PageExtractionResult {
    pub page_num: u32,
    pub words: Vec<Word>,
    pub paragraphs: Vec<Paragraph>,
}

/// Extract multiple pages in parallel
///
/// **Important Performance Notes:**
/// - Since MuPDF's Document type is not thread-safe (`!Sync`), each thread must open
///   its own copy of the document
/// - This creates overhead that can outweigh parallelization benefits for small to
///   moderate page counts
/// - This function uses chunking to batch pages per thread, amortizing document
///   opening cost across multiple pages
/// - Best use cases:
///   - Very large documents (100+ pages)
///   - When combined with streaming to process batches
///   - Future optimization: could be improved with a thread-safe document pool
///
/// For small documents (<20 pages), sequential extraction may be faster.
///
/// # Example
/// ```no_run
/// use voxpdf_core::{PDFDocument, extraction::extract_pages_parallel};
///
/// let doc = PDFDocument::open("example.pdf")?;
/// let pages = vec![0, 1, 2, 3, 4, 5];
/// let results = extract_pages_parallel(&doc, pages)?;
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn extract_pages_parallel(
    doc: &PDFDocument,
    page_nums: Vec<u32>,
) -> Result<Vec<PageExtractionResult>> {
    let doc_path = doc.path().to_string();

    // Use chunks to batch pages per thread
    // This amortizes the cost of opening the document
    let chunk_size = (page_nums.len() / rayon::current_num_threads()).max(1);

    let results: Result<Vec<Vec<PageExtractionResult>>> = page_nums
        .par_chunks(chunk_size)
        .map(|chunk| {
            // Each thread opens one copy of the document
            // and processes multiple pages with it
            let thread_doc = PDFDocument::open(&doc_path)?;

            let mut chunk_results = Vec::with_capacity(chunk.len());
            for &page_num in chunk {
                let words = extract_word_positions(&thread_doc, page_num)?;
                let mut paragraphs = detect_paragraphs(words.clone());
                paragraphs = reassemble_hyphenated_words(paragraphs);

                chunk_results.push(PageExtractionResult {
                    page_num,
                    words,
                    paragraphs,
                });
            }

            Ok(chunk_results)
        })
        .collect();

    // Flatten the results
    Ok(results?.into_iter().flatten().collect())
}
