import XCTest
@testable import VoxPDF

final class PDFDocumentTests: XCTestCase {
    func testOpenSimplePDF() throws {
        // Copy test PDF from Rust fixtures
        let testPDFPath = "../../voxpdf-core/tests/fixtures/simple.pdf"
        let url = URL(fileURLWithPath: testPDFPath)

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
        let testPDFPath = "../../voxpdf-core/tests/fixtures/simple.pdf"
        let url = URL(fileURLWithPath: testPDFPath)

        let doc = try PDFDocument(url: url)
        let text = try doc.text(page: 0)

        XCTAssertTrue(text.contains("Hello"))
        XCTAssertTrue(text.contains("World"))
    }
}
