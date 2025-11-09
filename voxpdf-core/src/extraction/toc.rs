use crate::error::Result;
use crate::models::TocEntry;
use crate::pdf::PDFDocument;

pub fn extract_toc(doc: &PDFDocument) -> Result<Vec<TocEntry>> {
    // Try mupdf first (fast, production-grade)
    let outlines = match doc.doc.outlines() {
        Ok(outlines) => outlines,
        Err(_) => {
            // mupdf failed - try lopdf fallback if available
            #[cfg(feature = "toc-fallback")]
            {
                return extract_toc_lopdf_fallback(doc);
            }

            #[cfg(not(feature = "toc-fallback"))]
            {
                return Ok(Vec::new()); // No fallback available
            }
        }
    };

    // Flatten the hierarchical outline into a list of TOC entries
    let mut entries = Vec::new();
    flatten_outlines(&outlines, 0, &mut entries);

    Ok(entries)
}

/// Fallback TOC extraction using lopdf (more lenient with corrupted outlines)
#[cfg(feature = "toc-fallback")]
fn extract_toc_lopdf_fallback(doc: &PDFDocument) -> Result<Vec<TocEntry>> {
    use lopdf::Document as LoPdfDocument;

    // Load PDF with lopdf
    let lopdf_doc = LoPdfDocument::load(&doc.path())?;

    // Get catalog and outlines
    let catalog = lopdf_doc.catalog()?;
    let outlines_ref = match catalog.get(b"Outlines") {
        Ok(r) => r,
        Err(_) => return Ok(Vec::new()), // No outlines
    };

    let outlines_id = outlines_ref.as_reference()?;
    let outlines_dict = lopdf_doc.get_dictionary(outlines_id)?;

    // Get first outline entry
    let first_ref = match outlines_dict.get(b"First") {
        Ok(r) => r,
        Err(_) => return Ok(Vec::new()), // Empty outlines
    };

    let first_id = first_ref.as_reference()?;

    // Traverse outline tree
    let mut entries = Vec::new();
    traverse_lopdf_outline(&lopdf_doc, first_id, 0, &mut entries)?;

    Ok(entries)
}

#[cfg(feature = "toc-fallback")]
fn traverse_lopdf_outline(
    doc: &lopdf::Document,
    outline_id: lopdf::ObjectId,
    level: u8,
    entries: &mut Vec<TocEntry>,
) -> Result<()> {
    let outline_dict = doc.get_dictionary(outline_id)?;

    // Extract title
    if let Ok(title_obj) = outline_dict.get(b"Title") {
        if let Ok(title_bytes) = title_obj.as_str() {
            let title = String::from_utf8_lossy(title_bytes).to_string();

            // Extract destination page (if available)
            let page_number = extract_page_from_dest(doc, &outline_dict).unwrap_or(0);

            entries.push(TocEntry::new(title, level, page_number, 0));
        }
    }

    // Traverse children (First)
    if let Ok(first_ref) = outline_dict.get(b"First") {
        if let Ok(first_id) = first_ref.as_reference() {
            traverse_lopdf_outline(doc, first_id, level + 1, entries)?;
        }
    }

    // Traverse siblings (Next)
    if let Ok(next_ref) = outline_dict.get(b"Next") {
        if let Ok(next_id) = next_ref.as_reference() {
            traverse_lopdf_outline(doc, next_id, level, entries)?;
        }
    }

    Ok(())
}

#[cfg(feature = "toc-fallback")]
fn extract_page_from_dest(doc: &lopdf::Document, outline_dict: &lopdf::Dictionary) -> Option<u32> {
    // Try to get Dest entry
    if let Ok(dest) = outline_dict.get(b"Dest") {
        // Dest can be a name, string, or array
        if let Ok(dest_array) = dest.as_array() {
            if let Some(page_ref) = dest_array.first() {
                if let Ok(page_id) = page_ref.as_reference() {
                    // Look up page number from page ID
                    let pages = doc.get_pages();
                    for (page_num, &id) in pages.iter() {
                        if id == page_id {
                            return Some(page_num.saturating_sub(1)); // Convert to 0-indexed
                        }
                    }
                }
            }
        }
    }

    // Try A (Action) dictionary as fallback
    if let Ok(action) = outline_dict.get(b"A") {
        if let Ok(action_dict) = action.as_dict() {
            if let Ok(dest) = action_dict.get(b"D") {
                if let Ok(dest_array) = dest.as_array() {
                    if let Some(page_ref) = dest_array.first() {
                        if let Ok(page_id) = page_ref.as_reference() {
                            let pages = doc.get_pages();
                            for (page_num, &id) in pages.iter() {
                                if id == page_id {
                                    return Some(page_num.saturating_sub(1));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    None
}

/// Recursively flatten the outline tree into a flat list of TOC entries
fn flatten_outlines(
    outlines: &[mupdf::Outline],
    level: u8,
    entries: &mut Vec<TocEntry>,
) {
    for outline in outlines {
        // Create TOC entry from this outline entry
        let entry = TocEntry::new(
            outline.title.clone(),
            level,
            outline.page.unwrap_or(0),
            0, // TODO: Map to paragraph_index when we have full document extraction
        );
        entries.push(entry);

        // Recursively process children
        if !outline.down.is_empty() {
            flatten_outlines(&outline.down, level + 1, entries);
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
