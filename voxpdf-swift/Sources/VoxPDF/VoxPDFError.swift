import Foundation

public enum VoxPDFError: Error {
    case invalidPDF
    case pageNotFound
    case ioError
    case outOfMemory
    case invalidText
    case unknown

    init(code: Int32) {
        switch code {
        case 1: self = .invalidPDF
        case 2: self = .pageNotFound
        case 3: self = .ioError
        case 4: self = .outOfMemory
        case 5: self = .invalidText
        default: self = .unknown
        }
    }
}
