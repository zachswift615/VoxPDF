use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use crate::error::VoxPDFError;
use crate::pdf::PDFDocument;

// Error codes for FFI
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CVoxPDFError {
    Ok = 0,
    InvalidPDF = 1,
    PageNotFound = 2,
    IoError = 3,
    OutOfMemory = 4,
}

impl From<VoxPDFError> for CVoxPDFError {
    fn from(err: VoxPDFError) -> Self {
        match err {
            VoxPDFError::InvalidPDF(_) => CVoxPDFError::InvalidPDF,
            VoxPDFError::PageNotFound(_, _) => CVoxPDFError::PageNotFound,
            VoxPDFError::Io(_) => CVoxPDFError::IoError,
            VoxPDFError::Lopdf(_) => CVoxPDFError::InvalidPDF,
            VoxPDFError::ExtractionError(_) => CVoxPDFError::InvalidPDF,
        }
    }
}

// Opaque pointer for PDFDocument
#[repr(C)]
pub struct CVoxPDFDocument {
    _private: [u8; 0],
}

// Open PDF document
#[no_mangle]
pub extern "C" fn voxpdf_open(
    path: *const c_char,
    error_out: *mut CVoxPDFError,
) -> *mut CVoxPDFDocument {
    if path.is_null() || error_out.is_null() {
        return std::ptr::null_mut();
    }

    let path_str = unsafe {
        match CStr::from_ptr(path).to_str() {
            Ok(s) => s,
            Err(_) => {
                *error_out = CVoxPDFError::InvalidPDF;
                return std::ptr::null_mut();
            }
        }
    };

    match PDFDocument::open(path_str) {
        Ok(doc) => {
            unsafe { *error_out = CVoxPDFError::Ok; }
            Box::into_raw(Box::new(doc)) as *mut CVoxPDFDocument
        }
        Err(e) => {
            unsafe { *error_out = e.into(); }
            std::ptr::null_mut()
        }
    }
}

// Get page count
#[no_mangle]
pub extern "C" fn voxpdf_get_page_count(doc: *const CVoxPDFDocument) -> usize {
    if doc.is_null() {
        return 0;
    }

    let doc = unsafe { &*(doc as *const PDFDocument) };
    doc.page_count()
}

// Free document
#[no_mangle]
pub extern "C" fn voxpdf_free_document(doc: *mut CVoxPDFDocument) {
    if !doc.is_null() {
        unsafe {
            let _ = Box::from_raw(doc as *mut PDFDocument);
        }
    }
}

// Extract page text
#[no_mangle]
pub extern "C" fn voxpdf_extract_page_text(
    doc: *const CVoxPDFDocument,
    page: u32,
    text_out: *mut *const c_char,
    error_out: *mut CVoxPDFError,
) -> bool {
    if doc.is_null() || text_out.is_null() || error_out.is_null() {
        return false;
    }

    let doc = unsafe { &*(doc as *const PDFDocument) };

    match crate::extraction::extract_page_text(doc, page) {
        Ok(text) => {
            match CString::new(text) {
                Ok(c_str) => {
                    unsafe {
                        *text_out = c_str.into_raw();
                        *error_out = CVoxPDFError::Ok;
                    }
                    true
                }
                Err(_) => {
                    unsafe { *error_out = CVoxPDFError::InvalidPDF; }
                    false
                }
            }
        }
        Err(e) => {
            unsafe { *error_out = e.into(); }
            false
        }
    }
}

// Free string
#[no_mangle]
pub extern "C" fn voxpdf_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = CString::from_raw(s);
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

        let doc = voxpdf_open(path.as_ptr(), &mut error);
        assert!(!doc.is_null());
        assert_eq!(error, CVoxPDFError::Ok);

        let count = voxpdf_get_page_count(doc);
        assert_eq!(count, 1);

        voxpdf_free_document(doc);
    }

    #[test]
    fn test_ffi_text_extraction() {
        let path = CString::new("tests/fixtures/simple.pdf").unwrap();
        let mut error = CVoxPDFError::Ok;

        let doc = voxpdf_open(path.as_ptr(), &mut error);
        assert!(!doc.is_null());

        let mut text_ptr: *const c_char = std::ptr::null();
        let result = voxpdf_extract_page_text(doc, 0, &mut text_ptr, &mut error);

        assert!(result);
        assert_eq!(error, CVoxPDFError::Ok);
        assert!(!text_ptr.is_null());

        let text = unsafe { CStr::from_ptr(text_ptr).to_string_lossy() };
        assert!(text.contains("Hello"));

        voxpdf_free_string(text_ptr as *mut c_char);
        voxpdf_free_document(doc);
    }
}
