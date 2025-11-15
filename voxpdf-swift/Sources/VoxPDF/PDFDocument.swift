import Foundation
import CVoxPDF

public class PDFDocument {
    private let handle: OpaquePointer

    public var pageCount: Int {
        Int(voxpdf_get_page_count(handle))
    }

    public init(url: URL) throws {
        var error: CVoxPDFError = CVoxPDFErrorOk

        guard let handle = voxpdf_open(url.path, &error) else {
            throw VoxPDFError(code: Int32(error.rawValue), context: url.path)
        }

        if error.rawValue != 0 {
            throw VoxPDFError(code: Int32(error.rawValue), context: url.path)
        }

        self.handle = handle
    }

    deinit {
        voxpdf_free_document(handle)
    }

    public func text(page: Int) throws -> String {
        var textPtr: UnsafePointer<CChar>?
        var error: CVoxPDFError = CVoxPDFErrorOk

        let result = voxpdf_extract_page_text(
            handle,
            UInt32(page),
            &textPtr,
            &error
        )

        guard result, error.rawValue == 0, let ptr = textPtr else {
            if error.rawValue == 2 {
                // Page not found - include context
                throw VoxPDFError.pageNotFound(page: page, totalPages: self.pageCount)
            }
            throw VoxPDFError(code: Int32(error.rawValue), context: "page \(page)")
        }

        defer { voxpdf_free_string(UnsafeMutablePointer(mutating: ptr)) }

        return String(cString: ptr)
    }

    public func wordPositions(page: Int) throws -> [Word] {
        var error: CVoxPDFError = CVoxPDFErrorOk

        let count = voxpdf_get_word_count(handle, UInt32(page), &error)
        guard error.rawValue == 0 else {
            if error.rawValue == 2 {
                throw VoxPDFError.pageNotFound(page: page, totalPages: self.pageCount)
            }
            throw VoxPDFError(code: Int32(error.rawValue), context: "page \(page)")
        }

        var words: [Word] = []
        words.reserveCapacity(count)

        for index in 0..<count {
            var cPosition = CWordPosition(x: 0, y: 0, width: 0, height: 0, page: 0, font_size: 0)
            var textPtr: UnsafePointer<CChar>?

            let result = voxpdf_get_word(
                handle,
                UInt32(page),
                index,
                &cPosition,
                &textPtr,
                &error
            )

            guard result, error.rawValue == 0, let ptr = textPtr else {
                if error.rawValue == 2 {
                    throw VoxPDFError.pageNotFound(page: page, totalPages: self.pageCount)
                }
                throw VoxPDFError(code: Int32(error.rawValue), context: "page \(page), word \(index)")
            }

            let text = String(cString: ptr)
            voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

            words.append(Word(text: text, cPosition: cPosition))
        }

        return words
    }

    /// Extracts paragraphs from a specific page.
    ///
    /// - Parameter page: The zero-indexed page number
    /// - Returns: Array of paragraphs with their text
    /// - Throws: `VoxPDFError` if extraction fails
    public func paragraphs(page: Int) throws -> [Paragraph] {
        var error: CVoxPDFError = CVoxPDFErrorOk

        let count = voxpdf_get_paragraph_count(handle, UInt32(page), &error)
        guard error.rawValue == 0 else {
            if error.rawValue == 2 {
                throw VoxPDFError.pageNotFound(page: page, totalPages: self.pageCount)
            }
            throw VoxPDFError(code: Int32(error.rawValue), context: "page \(page)")
        }

        var paragraphs: [Paragraph] = []
        paragraphs.reserveCapacity(count)

        for index in 0..<count {
            var cPara = CParagraph(index: 0, page_number: 0, word_count: 0)
            var textPtr: UnsafePointer<CChar>?

            let result = voxpdf_get_paragraph(
                handle,
                UInt32(page),
                index,
                &cPara,
                &textPtr,
                &error
            )

            guard result, error.rawValue == 0, let ptr = textPtr else {
                if error.rawValue == 2 {
                    throw VoxPDFError.pageNotFound(page: page, totalPages: self.pageCount)
                }
                throw VoxPDFError(code: Int32(error.rawValue), context: "page \(page), paragraph \(index)")
            }

            let text = String(cString: ptr)
            voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

            paragraphs.append(Paragraph(text: text, cParagraph: cPara))
        }

        return paragraphs
    }

    /// Extracts the table of contents from the document.
    ///
    /// - Returns: Array of TOC entries (empty if document has no TOC)
    /// - Throws: `VoxPDFError` if extraction fails
    public func tableOfContents() throws -> [TocEntry] {
        var error: CVoxPDFError = CVoxPDFErrorOk

        let count = voxpdf_get_toc_count(handle, &error)
        guard error.rawValue == 0 else {
            throw VoxPDFError(code: Int32(error.rawValue), context: "TOC extraction")
        }

        var entries: [TocEntry] = []
        entries.reserveCapacity(count)

        for index in 0..<count {
            var cToc = CTocEntry(level: 0, page_number: 0, paragraph_index: 0)
            var titlePtr: UnsafePointer<CChar>?

            let result = voxpdf_get_toc_entry(
                handle,
                index,
                &cToc,
                &titlePtr,
                &error
            )

            guard result, error.rawValue == 0, let ptr = titlePtr else {
                throw VoxPDFError(code: Int32(error.rawValue), context: "TOC entry \(index)")
            }

            let title = String(cString: ptr)
            voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

            entries.append(TocEntry(title: title, cTocEntry: cToc))
        }

        return entries
    }
}
