import Foundation
import CoreGraphics
import CVoxPDF

public struct Word {
    public let text: String
    public let bounds: CGRect
    public let pageNumber: Int

    init(text: String, cPosition: CWordPosition) {
        self.text = text
        self.bounds = CGRect(
            x: CGFloat(cPosition.x),
            y: CGFloat(cPosition.y),
            width: CGFloat(cPosition.width),
            height: CGFloat(cPosition.height)
        )
        self.pageNumber = Int(cPosition.page)
    }
}
