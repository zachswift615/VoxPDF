import Foundation

public enum VoxPDFError: Error {
    case invalidPDF(String?)
    case pageNotFound(page: Int?, totalPages: Int?)
    case ioError(String?)
    case outOfMemory
    case invalidText(context: String?)
    case unknown(code: Int32)

    init(code: Int32, context: String? = nil) {
        switch code {
        case 1: self = .invalidPDF(context)
        case 2: self = .pageNotFound(page: nil, totalPages: nil)
        case 3: self = .ioError(context)
        case 4: self = .outOfMemory
        case 5: self = .invalidText(context: context)
        default: self = .unknown(code: code)
        }
    }
}

extension VoxPDFError: LocalizedError {
    public var errorDescription: String? {
        switch self {
        case .invalidPDF(let context):
            if let context = context {
                return "Invalid PDF: \(context)"
            }
            return "Invalid PDF file"
        case .pageNotFound(let page, let totalPages):
            if let page = page, let totalPages = totalPages {
                return "Page \(page) not found (document has \(totalPages) pages)"
            } else if let page = page {
                return "Page \(page) not found"
            }
            return "Page not found"
        case .ioError(let context):
            if let context = context {
                return "I/O error: \(context)"
            }
            return "I/O error occurred"
        case .outOfMemory:
            return "Out of memory"
        case .invalidText(let context):
            if let context = context {
                return "Invalid text data: \(context)"
            }
            return "Invalid text data"
        case .unknown(let code):
            return "Unknown error (code: \(code))"
        }
    }
}
