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
            throw VoxPDFError(code: Int32(error.rawValue))
        }

        if error.rawValue != 0 {
            throw VoxPDFError(code: Int32(error.rawValue))
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
            throw VoxPDFError(code: Int32(error.rawValue))
        }

        defer { voxpdf_free_string(UnsafeMutablePointer(mutating: ptr)) }

        return String(cString: ptr)
    }

    public func wordPositions(page: Int) throws -> [Word] {
        var error: CVoxPDFError = CVoxPDFErrorOk

        let count = voxpdf_get_word_count(handle, UInt32(page), &error)
        guard error.rawValue == 0 else {
            throw VoxPDFError(code: Int32(error.rawValue))
        }

        var words: [Word] = []
        words.reserveCapacity(count)

        for index in 0..<count {
            var cPosition = CWordPosition(x: 0, y: 0, width: 0, height: 0, page: 0)
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
                throw VoxPDFError(code: Int32(error.rawValue))
            }

            let text = String(cString: ptr)
            voxpdf_free_string(UnsafeMutablePointer(mutating: ptr))

            words.append(Word(text: text, cPosition: cPosition))
        }

        return words
    }
}
