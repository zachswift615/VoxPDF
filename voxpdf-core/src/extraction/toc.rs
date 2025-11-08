use crate::error::Result;
use crate::models::Chapter;
use crate::pdf::PDFDocument;

pub fn extract_toc(doc: &PDFDocument) -> Result<Vec<Chapter>> {
    // Extract outline tree from mupdf
    let outlines = match doc.doc.outlines() {
        Ok(outlines) => outlines,
        Err(_) => return Ok(Vec::new()), // No outline or corrupted = empty TOC
    };

    // Flatten the hierarchical outline into a list of chapters
    let mut chapters = Vec::new();
    flatten_outlines(&outlines, 0, &mut chapters);

    Ok(chapters)
}

/// Recursively flatten the outline tree into a flat list of chapters
fn flatten_outlines(
    outlines: &[mupdf::Outline],
    level: u8,
    chapters: &mut Vec<Chapter>,
) {
    for outline in outlines {
        // Create chapter from this outline entry
        let chapter = Chapter::new(
            outline.title.clone(),
            level,
            outline.page.unwrap_or(0),
            0, // TODO: Map to paragraph_index when we have full document extraction
        );
        chapters.push(chapter);

        // Recursively process children
        if !outline.down.is_empty() {
            flatten_outlines(&outline.down, level + 1, chapters);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_toc_extraction_unit() {
        // Unit tests for outline parsing logic
    }
}
