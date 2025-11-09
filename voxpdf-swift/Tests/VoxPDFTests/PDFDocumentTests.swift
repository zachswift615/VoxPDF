import XCTest
@testable import VoxPDF

final class PDFDocumentTests: XCTestCase {
    func testOpenSimplePDF() throws {
        let url = Bundle.module.url(
            forResource: "simple",
            withExtension: "pdf",
            subdirectory: "TestPDFs"
        )!

        let doc = try PDFDocument(url: url)
        XCTAssertEqual(doc.pageCount, 1)
    }

    func testOpenNonexistentPDF() {
        let url = URL(fileURLWithPath: "/nonexistent.pdf")

        XCTAssertThrowsError(try PDFDocument(url: url)) { error in
            XCTAssertTrue(error is VoxPDFError)
        }
    }

    func testExtractText() throws {
        let url = Bundle.module.url(
            forResource: "simple",
            withExtension: "pdf",
            subdirectory: "TestPDFs"
        )!

        let doc = try PDFDocument(url: url)
        let text = try doc.text(page: 0)

        XCTAssertTrue(text.contains("Hello"))
        XCTAssertTrue(text.contains("World"))
    }

    func testWordPositions() throws {
        let url = Bundle.module.url(
            forResource: "simple",
            withExtension: "pdf",
            subdirectory: "TestPDFs"
        )!

        let doc = try PDFDocument(url: url)
        let words = try doc.wordPositions(page: 0)

        XCTAssertGreaterThan(words.count, 0)

        // All words should have valid bounds
        for word in words {
            XCTAssertFalse(word.text.isEmpty)
            XCTAssertGreaterThan(word.bounds.width, 0)
            XCTAssertGreaterThan(word.bounds.height, 0)
        }

        // Should find "Hello" and "World"
        let hello = words.first { $0.text.contains("Hello") }
        let world = words.first { $0.text.contains("World") }

        XCTAssertNotNil(hello)
        XCTAssertNotNil(world)
    }

    func testParagraphs() throws {
        let url = Bundle.module.url(
            forResource: "simple",
            withExtension: "pdf",
            subdirectory: "TestPDFs"
        )!

        let doc = try PDFDocument(url: url)
        let paragraphs = try doc.paragraphs(page: 0)

        XCTAssertGreaterThan(paragraphs.count, 0)

        // All paragraphs should have valid data
        for para in paragraphs {
            XCTAssertFalse(para.text.isEmpty)
            XCTAssertGreaterThan(para.wordCount, 0)
        }

        // First paragraph should contain text
        XCTAssertTrue(paragraphs[0].text.contains("Hello"))
    }
}
