import Foundation
import CVoxPDF

/// An entry in the PDF's table of contents.
public struct TocEntry {
    /// The title of this TOC entry.
    public let title: String

    /// The nesting level (0 = chapter, 1 = section, etc.).
    public let level: Int

    /// The zero-indexed page number where this entry points.
    public let pageNumber: Int

    /// The paragraph index for navigation.
    public let paragraphIndex: Int

    /// Returns true if this is a top-level chapter (level 0).
    public var isChapter: Bool {
        level == 0
    }

    /// Returns true if this is a section (level 1).
    public var isSection: Bool {
        level == 1
    }

    init(title: String, cTocEntry: CTocEntry) {
        self.title = title
        self.level = Int(cTocEntry.level)
        self.pageNumber = Int(cTocEntry.page_number)
        self.paragraphIndex = cTocEntry.paragraph_index
    }
}
