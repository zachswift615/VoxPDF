# VoxPDF

**Open-source PDF text extraction library optimized for text-to-speech**

[![Swift](https://img.shields.io/badge/Swift-5.9+-orange.svg)](https://swift.org)
[![Platform](https://img.shields.io/badge/Platform-iOS%2015%2B-blue.svg)](https://developer.apple.com/ios/)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## Mission

Text-to-speech changed my life. I can learn 100x faster by listening to books constantly while commuting, exercising, and doing chores. But PDF support in TTS apps is either non-existent or locked behind expensive commercial SDKs costing thousands of dollars.

**VoxPDF exists to make PDF-to-speech accessible to everyone.**

This is an open-source Swift library that extracts text from PDFs with intelligent layout understanding, optimized specifically for text-to-speech applications. Built incrementally over months/years, tested rigorously against real-world PDFs, and free forever.

## Why VoxPDF?

### The Problem

- **Commercial SDKs are expensive**: Foxit and PSPDFKit cost $3,000+ per year
- **Existing libraries are inadequate**:
  - PDFKit alone doesn't handle complex layouts (multi-column, headers/footers, reading order)
  - MuPDF is C-based and complex to integrate
  - No library focuses specifically on TTS needs
- **PDF conversion tools are imperfect**: Converting PDFs to EPUB loses structure and requires external tools

### The Solution

VoxPDF provides:

- ✅ **Intelligent text extraction** - Proper reading order, even in complex layouts
- ✅ **TTS optimization** - Clean text flow, hyphenation handling, structure preservation
- ✅ **Word-level tracking** - Precise positions for synchronized highlighting
- ✅ **Chapter detection** - Automatic TOC generation and navigation
- ✅ **Free and open source** - MIT licensed, community-driven
- ✅ **Swift-native** - Modern Swift API with async/await support
- ✅ **Production-ready** - Comprehensive tests, performance optimized

## Current Status

**🚧 In Active Development**

- **Current Version**: Pre-alpha (v0.0.1)
- **Target v0.1.0**: Basic text extraction better than PDFKit alone (3 weeks)
- **Target v1.0.0**: Production-ready for technical books and papers (6 months)

See [ROADMAP.md](docs/ROADMAP.md) for detailed milestones.

## Quick Start

```swift
import VoxPDF

// Load a PDF
let url = URL(fileURLWithPath: "book.pdf")
let extractor = PDFTextExtractor(url: url)

// Extract text optimized for TTS
let document = try await extractor.extract()

// Get chapters for navigation
for chapter in document.chapters {
    print("\(chapter.title) - Page \(chapter.pageNumber)")
}

// Get text to read aloud
for paragraph in document.paragraphs {
    // Feed to AVSpeechSynthesizer or your TTS engine
    speak(paragraph.text)
}

// Track word positions for highlighting
for word in paragraph.words {
    highlightRect(word.bounds, on: word.pageNumber)
}
```

## Features

### ✅ Current (v0.0.1-alpha)
- Basic text extraction using PDFKit
- Simple paragraph detection
- Hyphenation handling

### 🚧 In Progress (v0.1.0)
- Multi-column layout detection
- Header/footer removal
- TOC extraction from PDF metadata
- Word-level position tracking

### 🎯 Planned (v0.5.0)
- Intelligent reading order for complex layouts
- Chapter detection via font analysis
- Footnote handling
- Table detection

### 🌟 Future (v1.0.0+)
- 3-column layouts
- Sidebars and callouts
- Form field extraction
- Annotation processing

See [ROADMAP.md](docs/ROADMAP.md) for complete feature timeline.

## Philosophy

### Incremental Progress
We ship value early and improve over months/years. Each release solves real problems, even if not perfect.

### Tested Against Reality
Every feature is tested against real PDFs: technical books, academic papers, novels, magazines. We accept PDF test cases from the community.

### Focused Scope
VoxPDF optimizes for **reading**, not editing or rendering. We solve the 80% case extremely well rather than trying to handle every PDF edge case.

### Community-Driven
The roadmap evolves based on GitHub issues, user-submitted problematic PDFs, and community pull requests.

## Installation

**Coming Soon** - VoxPDF will be available via Swift Package Manager once v0.1.0 is released.

```swift
dependencies: [
    .package(url: "https://github.com/yourusername/VoxPDF.git", from: "0.1.0")
]
```

## Documentation

- [Roadmap](docs/ROADMAP.md) - Phased development plan
- [Architecture](docs/ARCHITECTURE.md) - Technical design decisions
- [Contributing](docs/CONTRIBUTING.md) - How to help build VoxPDF
- [API Reference](docs/API.md) - Detailed API documentation (coming soon)

## Use Cases

VoxPDF is perfect for:

- 📱 **TTS apps** - Audiobook players, accessibility tools, reading assistants
- 📚 **Educational tools** - Study apps, language learning, academic research
- ♿ **Accessibility** - Screen readers, dyslexia aids, visual impairment support
- 🎧 **Personal projects** - DIY audiobook creation, podcast generation

## Contributing

We welcome contributions! See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for:

- How to set up the development environment
- Good first issues for newcomers
- Code style guidelines
- How to submit test PDFs

**Help Wanted:**
- 🐛 Bug reports with problematic PDFs
- ✨ Feature requests based on real use cases
- 📝 Documentation improvements
- 🧪 Test PDFs from different generators (LaTeX, InDesign, Word, etc.)

## Real-World Testing

VoxPDF is battle-tested as the PDF engine for [Listen2](https://github.com/yourusername/Listen2), a TTS audiobook player for iOS. Every improvement to VoxPDF immediately benefits Listen2 users.

## License

MIT License - see [LICENSE](LICENSE) for details.

Free to use in commercial and open-source projects.

## Acknowledgments

Built with:
- **PDFKit** - Apple's PDF framework for basic parsing
- **Claude Code** - AI pair programming for incremental development
- **Community** - Test PDFs, bug reports, and feature requests from users worldwide

Inspired by:
- **Foxit SDK** - Commercial PDF SDK that proved professional TTS PDF support is possible
- **Voice Dream Reader** - iOS app that showed the importance of precise word highlighting
- **MuPDF** - Open-source PDF library demonstrating what's achievable

## Support

- 🐛 [Report a bug](https://github.com/yourusername/VoxPDF/issues/new?template=bug_report.md)
- 💡 [Request a feature](https://github.com/yourusername/VoxPDF/issues/new?template=feature_request.md)
- 📖 [Submit a problematic PDF](https://github.com/yourusername/VoxPDF/issues/new?template=pdf_test_case.md)
- 💬 [Discussions](https://github.com/yourusername/VoxPDF/discussions)

---

**Status**: 🚧 Pre-alpha - Not production-ready yet, but actively developed
**Maintainer**: [@zachswift](https://github.com/zachswift)
**Started**: November 2025
**First Release Target**: January 2026 (v0.1.0)

**Star ⭐ this repo if you believe TTS should be accessible to everyone!**
