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
}
