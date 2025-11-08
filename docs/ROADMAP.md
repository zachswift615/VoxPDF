# VoxPDF Development Roadmap

**Vision**: Build an open-source PDF text extraction library competitive with commercial SDKs like Foxit, optimized for text-to-speech applications.

**Timeline**: Incremental development over 12+ months
**Philosophy**: Ship value early, iterate based on real-world testing

---

## Milestone Overview

| Version | Target | Status | Focus |
|---------|--------|--------|-------|
| v0.1.0 | Jan 2026 | 🚧 In Progress | Better than PDFKit alone |
| v0.3.0 | Mar 2026 | 📋 Planned | Technical books support |
| v0.5.0 | Jun 2026 | 📋 Planned | Complex layouts |
| v1.0.0 | Dec 2026 | 🎯 Goal | Production ready |
| v2.0.0 | TBD | 💭 Vision | Foxit feature parity |

---

## Phase 0: Foundation (Week 1)
**Target: Nov 11-15, 2025**

### Goals
- ✅ Project manifesto and documentation
- ✅ Repository structure
- ✅ CI/CD pipeline
- ✅ Community guidelines

### Deliverables
- [x] README.md with mission statement
- [x] ROADMAP.md (this document)
- [ ] CONTRIBUTING.md
- [ ] ARCHITECTURE.md
- [ ] LICENSE (MIT)
- [ ] GitHub issue templates
- [ ] GitHub Actions for CI
- [ ] Swift Package Manager structure

### Success Criteria
- Can clone repo and understand the mission
- Can run initial tests (even if minimal)
- Clear path for contributors

---

## Phase 1: MVP - Better Than PDFKit (Weeks 2-4)
**Target: Nov 18 - Dec 6, 2025**
**Version: v0.1.0**

### Goals
Create a library that Listen2 can use that's **demonstrably better** than hand-rolled PDFKit code.

### Core Features

#### 1. Smart Text Extraction
- ✅ Use PDFKit for PDF access
- ✅ Extract text page-by-page
- ✅ Preserve word boundaries
- ✅ Handle Unicode correctly
- ✅ Clean whitespace intelligently

#### 2. Paragraph Detection
- ✅ Join lines within paragraphs properly
- ✅ Detect paragraph boundaries by:
  - Line spacing
  - Indentation changes
  - Sentence-ending punctuation
- ✅ Avoid merging headers with body text

#### 3. Hyphenation Handling
- ✅ Detect hyphenated words across line breaks
- ✅ Reassemble hyphenated words (e.g., "inter-\nrupt" → "interrupt")
- ✅ Preserve intentional hyphens (e.g., "self-contained")
- ✅ Handle different hyphen characters (-, ‐, ‑)

#### 4. Word-Level Position Tracking
- ✅ Track exact CGRect for each word
- ✅ Map words to page numbers
- ✅ Enable synchronized highlighting
- ✅ Support efficient lookup by position

#### 5. Basic TOC Extraction
- ✅ Extract PDF outline metadata if available
- ✅ Convert to navigable chapter structure
- ✅ Map chapters to approximate paragraph indices
- ✅ Fallback: empty TOC if no metadata

### API Design

```swift
// Core types
struct PDFDocument {
    let url: URL
    let metadata: PDFMetadata
    let pages: [PDFPage]
    let paragraphs: [Paragraph]
    let chapters: [Chapter]
}

struct Paragraph {
    let id: UUID
    let text: String
    let pageNumber: Int
    let words: [Word]
}

struct Word {
    let text: String
    let bounds: CGRect
    let pageNumber: Int
}

struct Chapter {
    let title: String
    let level: Int // 0=chapter, 1=section, 2=subsection
    let paragraphIndex: Int
    let pageNumber: Int
}

// Main API
class PDFTextExtractor {
    init(url: URL)

    func extract() async throws -> PDFDocument
    func extractPage(_ pageNumber: Int) async throws -> PDFPage
}
```

### Testing Strategy
- Unit tests for hyphenation logic
- Unit tests for paragraph detection
- Integration tests with real PDFs:
  - O'Reilly technical books
  - Academic papers
  - Simple novels
- Benchmark against PDFKit baseline

### Success Criteria
- [ ] Extracts text from 10 test PDFs correctly
- [ ] Handles hyphenation in all test cases
- [ ] Paragraph boundaries are sensible (manual review)
- [ ] Word positions are accurate (±2px)
- [ ] Faster than 100ms per page
- [ ] Memory usage < 100MB for 500-page PDF
- [ ] **Listen2 can integrate and see improvement**

---

## Phase 2: Technical Books (Months 2-3)
**Target: Dec 2025 - Feb 2026**
**Version: v0.3.0**

### Goals
Handle **technical books and academic papers** well - the 80% use case for TTS.

### Features

#### 1. Multi-Column Detection (2 columns)
- Use whitespace analysis to detect columns
- Determine proper reading order (left-to-right, top-to-bottom)
- Handle page breaks within columns
- Detect single-column pages in multi-column documents

**Algorithm**: XY-Cut recursive segmentation

#### 2. Header/Footer Removal
- Detect repeating elements across pages
- Identify position-based patterns (top/bottom margins)
- Remove page numbers
- Filter running headers
- Preserve footnotes (different detection)

#### 3. Chapter Detection via Font Analysis
- Analyze font sizes across document
- Identify heading hierarchy (H1, H2, H3)
- Detect bold/italic as heading indicators
- Build TOC from detected headings
- Confidence scoring for each detected heading

#### 4. Footnote Handling
- Detect superscript references
- Link references to footnotes
- Option to include/exclude footnotes in reading
- Preserve footnote text for reference

#### 5. Figure/Table Detection
- Detect image regions
- Identify table structures (grid patterns)
- Extract captions
- Option to skip or read caption text
- Preserve reading flow around figures

### API Additions

```swift
struct ExtractionOptions {
    var includeHeaders: Bool = false
    var includeFooters: Bool = false
    var includeFootnotes: Bool = true
    var readTableCaptions: Bool = true
    var readFigureCaptions: Bool = true
}

class PDFTextExtractor {
    var options: ExtractionOptions

    func detectColumns(page: Int) -> ColumnLayout
    func detectHeadings() -> [Chapter]
}

enum ColumnLayout {
    case single
    case twoColumn
    case threeColumn
    case mixed
}
```

### Testing PDFs
- LaTeX-generated papers (arXiv)
- O'Reilly technical books
- Academic journals
- Textbooks with figures/tables

### Success Criteria
- [ ] 2-column papers read in correct order
- [ ] Headers/footers removed accurately (>95%)
- [ ] Chapters detected from font analysis
- [ ] Footnotes properly linked
- [ ] Tables don't break paragraph flow

---

## Phase 3: Complex Layouts (Months 4-6)
**Target: Mar - Jun 2026**
**Version: v0.5.0**

### Features

#### 1. Advanced Column Detection
- 3-column layouts
- Mixed layouts (single + multi-column on same page)
- Newspaper-style flowing text
- Variable column widths

#### 2. Sidebar/Callout Detection
- Detect boxes/borders
- Identify shaded regions
- Recognize callout boxes
- Determine if sidebar is inline or floating

#### 3. List Recognition
- Bullet point detection
- Numbered list recognition
- Nested lists
- Indentation-based hierarchy

#### 4. Table Extraction
- Grid detection
- Row/column extraction
- Cell content mapping
- Table-to-text strategies:
  - Row-by-row reading
  - Column-aware reading
  - Skip and read caption only

#### 5. Reading Order Optimization
- Proximity-based text flow
- Semantic grouping of text blocks
- Handle wraparound text
- Z-pattern vs F-pattern detection

### Success Criteria
- [ ] Magazines read correctly
- [ ] Complex textbooks handled
- [ ] Sidebars don't break flow
- [ ] Lists preserve structure

---

## Phase 4: Production Ready (Months 7-9)
**Target: Jul - Sep 2026**
**Version: v1.0.0**

### Focus: Performance, Reliability, Polish

#### 1. Performance Optimization
- Lazy page loading
- Background extraction
- Concurrent processing
- LRU caching system
- Memory-mapped file support
- **Target**: <50ms per page, <50MB for 500-page PDF

#### 2. Error Handling
- Graceful degradation for malformed PDFs
- Clear error messages
- Partial extraction on failure
- Validation and recovery

#### 3. Comprehensive Testing
- Unit test coverage >90%
- Integration tests with 100+ PDFs
- Performance benchmarks
- Stress testing (1000+ page PDFs)
- Memory leak detection
- Thread safety verification

#### 4. Documentation
- Complete API documentation (DocC)
- Usage guides and examples
- Migration guide from PDFKit
- Best practices
- Performance tuning guide

#### 5. API Stability
- Semantic versioning commitment
- Deprecation policy
- Backward compatibility guarantees

### Success Criteria
- [ ] No crashes on 1000 test PDFs
- [ ] Performance meets targets
- [ ] Full API documentation
- [ ] **10+ apps using VoxPDF in production**

---

## Phase 5: Advanced Features (Months 10-12)
**Target: Oct - Dec 2026**
**Version: v1.5.0**

### Features

#### 1. Form Field Extraction
- Text field content
- Checkbox states
- Radio button values
- Dropdown selections

#### 2. Annotation Processing
- Highlight extraction
- Comment/note reading
- Markup interpretation
- Annotation metadata

#### 3. Link Processing
- Internal link detection
- External URL extraction
- Cross-reference handling
- Bookmark navigation

#### 4. Metadata Enhancement
- Document properties
- Author/title/subject
- Keywords
- Custom metadata fields

#### 5. Accessibility Features
- Tagged PDF support
- Logical structure tree
- Alternative descriptions
- WCAG compliance

---

## Phase 6: Foxit Parity (Year 2+)
**Target: 2027+**
**Version: v2.0.0**

### Advanced Capabilities

#### 1. Image Text Extraction
- OCR integration hooks
- Image alt text
- Figure analysis
- Chart/graph interpretation

#### 2. Mathematical Content
- LaTeX/MathML detection
- Equation reading strategies
- Symbol pronunciation
- Formula linearization

#### 3. Multi-Language Support
- RTL (Right-to-Left) text
- CJK (Chinese, Japanese, Korean)
- Mixed language documents
- Language detection

#### 4. Advanced Layout
- Flowing text around images
- Multi-path reading orders
- Adaptive layout understanding
- Magazine-style complex layouts

#### 5. Commercial-Grade Features
- Encrypted PDF support
- Digital signatures
- Redaction detection
- Watermark handling

---

## Community Milestones

### Early Adopters (Q1 2026)
- 5+ GitHub stars
- 2+ external contributors
- 10+ issues/PRs
- Featured in at least one iOS TTS app

### Growing Community (Q2 2026)
- 50+ GitHub stars
- 10+ contributors
- Used in 5+ production apps
- Regular community PRs

### Established Project (Q4 2026)
- 200+ GitHub stars
- Active community discussions
- Monthly releases
- Cited in blogs/articles
- Conference talk about the project

---

## Success Metrics

### Technical Metrics
- **Accuracy**: >95% correct reading order on test suite
- **Performance**: <100ms per page extraction
- **Memory**: <50MB for 500-page PDF
- **Coverage**: >90% unit test coverage
- **Stability**: <1% crash rate in production

### Adoption Metrics
- **GitHub stars**: 200+ by v1.0
- **Production apps**: 10+ by v1.0
- **Contributors**: 20+ by v1.0
- **Test PDFs submitted**: 100+ by v1.0

### Community Metrics
- **Issues closed**: >80% within 2 weeks
- **PR review time**: <3 days average
- **Active contributors**: 5+ monthly by v1.0
- **Documentation quality**: High (community feedback)

---

## Risk Mitigation

### Technical Risks

**Risk**: PDF spec is too complex
- **Mitigation**: Focus on common cases, graceful degradation, accept imperfection

**Risk**: Performance doesn't meet targets
- **Mitigation**: Profile early, optimize hot paths, consider C++ for critical code

**Risk**: Layout detection is inaccurate
- **Mitigation**: Comprehensive test suite, confidence scoring, manual review option

### Community Risks

**Risk**: Low adoption
- **Mitigation**: Excellent documentation, demo apps, active marketing

**Risk**: Lack of contributors
- **Mitigation**: Good first issues, welcoming community, clear contribution guide

**Risk**: Scope creep
- **Mitigation**: Strict roadmap, say no to features outside TTS focus

---

## Flexibility

This roadmap is a **living document**. Priorities will shift based on:

- Real-world PDF test cases from community
- Listen2 usage data and pain points
- Contributor interests and PRs
- User feedback and feature requests
- Performance bottlenecks discovered

**Principle**: Ship value early and often. Perfect is the enemy of good.

---

**Last Updated**: November 7, 2025
**Next Review**: January 1, 2026
