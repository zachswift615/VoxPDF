use crate::error::VoxPDFError;
use crate::pdf::PDFDocument;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// Error codes for FFI
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CVoxPDFError {
    Ok = 0,
    InvalidPDF = 1,
    PageNotFound = 2,
    IoError = 3,
    OutOfMemory = 4,
    InvalidText = 5,
}

impl From<VoxPDFError> for CVoxPDFError {
    fn from(err: VoxPDFError) -> Self {
        match err {
            VoxPDFError::InvalidPDF(_) => CVoxPDFError::InvalidPDF,
            VoxPDFError::PageNotFound(_, _) => CVoxPDFError::PageNotFound,
            VoxPDFError::Io(_) => CVoxPDFError::IoError,
            VoxPDFError::ExtractionError(_) => CVoxPDFError::InvalidPDF,
        }
    }
}

// Opaque pointer for PDFDocument
#[repr(C)]
pub struct CVoxPDFDocument {
    _private: [u8; 0],
}

/// C-compatible word position with bounding box.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CWordPosition {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub page: u32,
    pub font_size: f32,
}

/// C-compatible paragraph structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CParagraph {
    pub index: usize,
    pub page_number: u32,
    pub word_count: usize,
}

/// C-compatible TOC entry structure.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CTocEntry {
    pub level: u8,
    pub page_number: u32,
    pub paragraph_index: usize,
}

/// Open a PDF document from a file path.
///
/// # Safety
///
/// - `path` must be a valid null-terminated C string pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_document` on the returned pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_open(
    path: *const c_char,
    error_out: *mut CVoxPDFError,
) -> *mut CVoxPDFDocument {
    if path.is_null() || error_out.is_null() {
        return std::ptr::null_mut();
    }

    let path_str = match CStr::from_ptr(path).to_str() {
        Ok(s) => s,
        Err(_) => {
            *error_out = CVoxPDFError::InvalidPDF;
            return std::ptr::null_mut();
        }
    };

    match PDFDocument::open(path_str) {
        Ok(doc) => {
            *error_out = CVoxPDFError::Ok;
            Box::into_raw(Box::new(doc)) as *mut CVoxPDFDocument
        }
        Err(e) => {
            *error_out = e.into();
            std::ptr::null_mut()
        }
    }
}

/// Get the number of pages in a PDF document.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_page_count(doc: *const CVoxPDFDocument) -> usize {
    if doc.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);
    doc.page_count() as usize
}

/// Free a PDF document, releasing all associated resources.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been previously freed
/// - After calling this function, `doc` must not be used again
#[no_mangle]
pub unsafe extern "C" fn voxpdf_free_document(doc: *mut CVoxPDFDocument) {
    if !doc.is_null() {
        let _ = Box::from_raw(doc as *mut PDFDocument);
    }
}

/// Extract text from a specific page of a PDF document.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `text_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned text pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_extract_page_text(
    doc: *const CVoxPDFDocument,
    page: u32,
    text_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || text_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_page_text(doc, page) {
        Ok(text) => match CString::new(text) {
            Ok(c_str) => {
                *text_out = c_str.into_raw();
                *error_out = CVoxPDFError::Ok;
                true
            }
            Err(_) => {
                *error_out = CVoxPDFError::InvalidText;
                false
            }
        },
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}

/// Free a string returned from `voxpdf_extract_page_text`.
///
/// # Safety
///
/// - `s` must be a valid pointer returned from `voxpdf_extract_page_text`
/// - `s` must not have been previously freed
/// - After calling this function, `s` must not be used again
#[no_mangle]
pub unsafe extern "C" fn voxpdf_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

/// Get the number of words on a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_word_count(
    doc: *const CVoxPDFDocument,
    page: u32,
    error_out: *mut CVoxPDFError,
) -> usize {
    if doc.is_null() || error_out.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            *error_out = CVoxPDFError::Ok;
            words.len()
        }
        Err(e) => {
            *error_out = e.into();
            0
        }
    }
}

/// Get a specific word by index from a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `word_out` must be a valid mutable pointer to CWordPosition
/// - `text_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned text pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_word(
    doc: *const CVoxPDFDocument,
    page: u32,
    index: usize,
    word_out: *mut CWordPosition,
    text_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || word_out.is_null() || text_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            if let Some(word) = words.get(index) {
                *word_out = CWordPosition {
                    x: word.bounds.x,
                    y: word.bounds.y,
                    width: word.bounds.width,
                    height: word.bounds.height,
                    page: word.page_number,
                    font_size: word.font_size,
                };

                match CString::new(word.text.clone()) {
                    Ok(c_str) => {
                        *text_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                        return true;
                    }
                    Err(_) => {
                        *error_out = CVoxPDFError::InvalidText;
                        return false;
                    }
                }
            }

            *error_out = CVoxPDFError::PageNotFound;
            false
        }
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}

/// Get the number of paragraphs on a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_paragraph_count(
    doc: *const CVoxPDFDocument,
    page: u32,
    error_out: *mut CVoxPDFError,
) -> usize {
    if doc.is_null() || error_out.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);

    // Extract words and detect paragraphs
    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            let paragraphs = crate::extraction::detect_paragraphs(words);
            *error_out = CVoxPDFError::Ok;
            paragraphs.len()
        }
        Err(e) => {
            *error_out = e.into();
            0
        }
    }
}

/// Get a specific paragraph by index from a page.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `para_out` must be a valid mutable pointer to CParagraph
/// - `text_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned text pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_paragraph(
    doc: *const CVoxPDFDocument,
    page: u32,
    index: usize,
    para_out: *mut CParagraph,
    text_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || para_out.is_null() || text_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    // Extract words and detect paragraphs
    match crate::extraction::extract_word_positions(doc, page) {
        Ok(words) => {
            let paragraphs = crate::extraction::detect_paragraphs(words);

            if let Some(para) = paragraphs.get(index) {
                *para_out = CParagraph {
                    index: para.index,
                    page_number: para.page_number,
                    word_count: para.word_count(),
                };

                match CString::new(para.text.clone()) {
                    Ok(c_str) => {
                        *text_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                        return true;
                    }
                    Err(_) => {
                        *error_out = CVoxPDFError::InvalidText;
                        return false;
                    }
                }
            }

            *error_out = CVoxPDFError::PageNotFound;
            false
        }
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}

/// Get the number of TOC entries in the document.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_toc_count(
    doc: *const CVoxPDFDocument,
    error_out: *mut CVoxPDFError,
) -> usize {
    if doc.is_null() || error_out.is_null() {
        return 0;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_toc(doc) {
        Ok(toc_entries) => {
            *error_out = CVoxPDFError::Ok;
            toc_entries.len()
        }
        Err(e) => {
            *error_out = e.into();
            0
        }
    }
}

/// Get a specific TOC entry by index.
///
/// # Safety
///
/// - `doc` must be a valid pointer returned from `voxpdf_open`
/// - `doc` must not have been freed with `voxpdf_free_document`
/// - `toc_out` must be a valid mutable pointer to CTocEntry
/// - `title_out` must be a valid mutable pointer
/// - `error_out` must be a valid mutable pointer to CVoxPDFError
/// - Caller must eventually call `voxpdf_free_string` on the returned title pointer
#[no_mangle]
pub unsafe extern "C" fn voxpdf_get_toc_entry(
    doc: *const CVoxPDFDocument,
    index: usize,
    toc_out: *mut CTocEntry,
    title_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || toc_out.is_null() || title_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = &*(doc as *const PDFDocument);

    match crate::extraction::extract_toc(doc) {
        Ok(toc_entries) => {
            if let Some(entry) = toc_entries.get(index) {
                *toc_out = CTocEntry {
                    level: entry.level,
                    page_number: entry.page_number,
                    paragraph_index: entry.paragraph_index,
                };

                match CString::new(entry.title.clone()) {
                    Ok(c_str) => {
                        *title_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                        return true;
                    }
                    Err(_) => {
                        *error_out = CVoxPDFError::InvalidText;
                        return false;
                    }
                }
            }

            *error_out = CVoxPDFError::PageNotFound;
            false
        }
        Err(e) => {
            *error_out = e.into();
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_document_lifecycle() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        unsafe {
            let doc = voxpdf_open(path.as_ptr(), &mut error);
            assert!(!doc.is_null());
            assert_eq!(error, CVoxPDFError::Ok);

            let count = voxpdf_get_page_count(doc);
            assert_eq!(count, 1);

            voxpdf_free_document(doc);
        }
    }

    #[test]
    fn test_ffi_text_extraction() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        unsafe {
            let doc = voxpdf_open(path.as_ptr(), &mut error);
            assert!(!doc.is_null());

            let mut text_ptr: *const c_char = std::ptr::null();
            let result = voxpdf_extract_page_text(doc, 0, &mut text_ptr, &mut error);

            assert!(result);
            assert_eq!(error, CVoxPDFError::Ok);
            assert!(!text_ptr.is_null());

            let text = CStr::from_ptr(text_ptr).to_string_lossy();
            assert!(text.contains("Hello"));

            voxpdf_free_string(text_ptr as *mut c_char);
            voxpdf_free_document(doc);
        }
    }

    #[test]
    fn test_ffi_word_positions() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        unsafe {
            let doc = voxpdf_open(path.as_ptr(), &mut error);
            assert!(!doc.is_null());

            let count = voxpdf_get_word_count(doc, 0, &mut error);
            assert!(count > 0);
            assert_eq!(error, CVoxPDFError::Ok);

            let mut word_pos = CWordPosition {
                x: 0.0,
                y: 0.0,
                width: 0.0,
                height: 0.0,
                page: 0,
                font_size: 0.0,
            };
            let mut text_ptr: *const c_char = std::ptr::null();

            let result = voxpdf_get_word(doc, 0, 0, &mut word_pos, &mut text_ptr, &mut error);
            assert!(result);
            assert!(word_pos.width > 0.0);
            assert!(!text_ptr.is_null());

            voxpdf_free_string(text_ptr as *mut c_char);
            voxpdf_free_document(doc);
        }
    }

    #[test]
    fn test_ffi_paragraphs() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        unsafe {
            let doc = voxpdf_open(path.as_ptr(), &mut error);
            assert!(!doc.is_null());

            let count = voxpdf_get_paragraph_count(doc, 0, &mut error);
            assert!(count > 0);
            assert_eq!(error, CVoxPDFError::Ok);

            let mut para = CParagraph {
                index: 0,
                page_number: 0,
                word_count: 0,
            };
            let mut text_ptr: *const c_char = std::ptr::null();

            let result = voxpdf_get_paragraph(doc, 0, 0, &mut para, &mut text_ptr, &mut error);
            assert!(result);
            assert!(para.word_count > 0);
            assert!(!text_ptr.is_null());

            voxpdf_free_string(text_ptr as *mut c_char);
            voxpdf_free_document(doc);
        }
    }

    #[test]
    fn test_ffi_toc() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        unsafe {
            let doc = voxpdf_open(path.as_ptr(), &mut error);
            assert!(!doc.is_null());

            // simple.pdf has no TOC, so should return 0
            let count = voxpdf_get_toc_count(doc, &mut error);
            assert_eq!(count, 0);
            assert_eq!(error, CVoxPDFError::Ok);

            voxpdf_free_document(doc);
        }
    }
}
