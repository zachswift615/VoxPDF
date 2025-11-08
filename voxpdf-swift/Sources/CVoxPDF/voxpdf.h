#ifndef VOXPDF_H
#define VOXPDF_H

#include <stdint.h>
#include <stdbool.h>

// Error codes
typedef enum {
    CVoxPDFErrorOk = 0,
    CVoxPDFErrorInvalidPDF = 1,
    CVoxPDFErrorPageNotFound = 2,
    CVoxPDFErrorIoError = 3,
    CVoxPDFErrorOutOfMemory = 4,
    CVoxPDFErrorInvalidText = 5,
} CVoxPDFError;

// Opaque pointer to document
typedef struct CVoxPDFDocument CVoxPDFDocument;

// Open PDF document
CVoxPDFDocument* voxpdf_open(const char* path, CVoxPDFError* error_out);

// Get page count
size_t voxpdf_get_page_count(const CVoxPDFDocument* doc);

// Extract page text
bool voxpdf_extract_page_text(
    const CVoxPDFDocument* doc,
    uint32_t page,
    const char** text_out,
    CVoxPDFError* error_out
);

// Free document
void voxpdf_free_document(CVoxPDFDocument* doc);

// Free string
void voxpdf_free_string(char* s);

#endif // VOXPDF_H
