#ifndef VOXPDF_H
#define VOXPDF_H

#include <stdint.h>
#include <stdbool.h>
#include <stddef.h>

// Error codes
typedef enum {
    CVoxPDFErrorOk = 0,
    CVoxPDFErrorInvalidPDF = 1,
    CVoxPDFErrorPageNotFound = 2,
    CVoxPDFErrorIoError = 3,
    CVoxPDFErrorOutOfMemory = 4,
    CVoxPDFErrorInvalidText = 5,
} CVoxPDFError;

// Opaque document pointer
typedef struct CVoxPDFDocument CVoxPDFDocument;

// Word position structure
typedef struct {
    float x;
    float y;
    float width;
    float height;
    uint32_t page;
} CWordPosition;

// Paragraph structure
typedef struct {
    size_t index;
    uint32_t page_number;
    size_t word_count;
} CParagraph;

// TOC entry structure
typedef struct {
    uint8_t level;
    uint32_t page_number;
    size_t paragraph_index;
} CTocEntry;

// Document lifecycle
CVoxPDFDocument* voxpdf_open(const char* path, CVoxPDFError* error_out);
size_t voxpdf_get_page_count(const CVoxPDFDocument* doc);
void voxpdf_free_document(CVoxPDFDocument* doc);

// Text extraction
bool voxpdf_extract_page_text(
    const CVoxPDFDocument* doc,
    uint32_t page,
    const char** text_out,
    CVoxPDFError* error_out
);

// Word positions
size_t voxpdf_get_word_count(
    const CVoxPDFDocument* doc,
    uint32_t page,
    CVoxPDFError* error_out
);

bool voxpdf_get_word(
    const CVoxPDFDocument* doc,
    uint32_t page,
    size_t index,
    CWordPosition* word_out,
    const char** text_out,
    CVoxPDFError* error_out
);

// Paragraphs
size_t voxpdf_get_paragraph_count(
    const CVoxPDFDocument* doc,
    uint32_t page,
    CVoxPDFError* error_out
);

bool voxpdf_get_paragraph(
    const CVoxPDFDocument* doc,
    uint32_t page,
    size_t index,
    CParagraph* para_out,
    const char** text_out,
    CVoxPDFError* error_out
);

// TOC
size_t voxpdf_get_toc_count(
    const CVoxPDFDocument* doc,
    CVoxPDFError* error_out
);

bool voxpdf_get_toc_entry(
    const CVoxPDFDocument* doc,
    size_t index,
    CTocEntry* toc_out,
    const char** title_out,
    CVoxPDFError* error_out
);

// Memory management
void voxpdf_free_string(char* s);

#endif // VOXPDF_H
