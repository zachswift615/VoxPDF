import Foundation
import CVoxPDF

/// A paragraph of text extracted from a PDF page.
public struct Paragraph {
    /// The index of this paragraph within the page.
    public let index: Int

    /// The text content of the paragraph.
    public let text: String

    /// The zero-indexed page number.
    public let pageNumber: Int

    /// The number of words in this paragraph.
    public let wordCount: Int

    init(text: String, cParagraph: CParagraph) {
        self.index = cParagraph.index
        self.text = text
        self.pageNumber = Int(cParagraph.page_number)
        self.wordCount = cParagraph.word_count
    }
}
