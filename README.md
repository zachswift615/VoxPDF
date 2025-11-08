# VoxPDF

**Cross-platform PDF text extraction library optimized for text-to-speech**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Platform](https://img.shields.io/badge/Platform-iOS%20%7C%20Android%20%7C%20Web-blue.svg)](https://github.com/yourusername/voxpdf)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

---

## Mission

Text-to-speech changed my life. I can learn 100x faster by listening to books constantly while commuting, exercising, and doing chores. But PDF support in TTS apps is either non-existent or locked behind expensive commercial SDKs costing thousands of dollars.

**VoxPDF exists to make PDF-to-speech accessible to everyone.**

This is an open-source, cross-platform library that extracts text from PDFs with intelligent layout understanding, optimized specifically for text-to-speech applications. Built with a **Rust core** and platform-specific bindings (Swift, Kotlin, WASM), following the proven architecture of commercial SDKs like Foxit and PSPDFKit. Built incrementally over months/years, tested rigorously against real-world PDFs, and free forever.

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
- ✅ **Cross-platform architecture** - Rust core + C FFI + platform bindings (Swift, Kotlin, WASM)
- ✅ **Commercial-grade design** - Same architecture as Foxit SDK and PSPDFKit
- ✅ **Performance** - Native Rust/C-level speed without sacrificing memory safety
- ✅ **Platform-native APIs** - Idiomatic Swift/Kotlin bindings with modern concurrency support

## Current Status

**🚧 In Active Development**

- **Current Phase**: Phase 0 (Foundation) - COMPLETE
  - ✅ Project structure initialized
  - ✅ Architecture documented (Rust core + FFI + bindings)
  - ✅ Development roadmap established

- **Next Step**: Slice 0 - lopdf validation spike (2-3 days)
  - Validate word position extraction capabilities
  - Decision point: Continue with lopdf or pivot to mupdf-sys

- **Target v0.1.0**: Basic text extraction better than PDFKit alone (4-6 weeks)
- **Target v1.0.0**: Production-ready for technical books and papers (9-12 months)

See [ROADMAP.md](docs/ROADMAP.md) for detailed milestones.

## Quick Start

**Note:** VoxPDF is still in early development. The API shown below represents the planned Swift bindings interface (wrapping the Rust core).

### iOS/macOS (Swift)

```swift
import VoxPDF

// Load a PDF
let url = URL(fileURLWithPath: "book.pdf")
let document = try PDFDocument(url: url)

// Extract text optimized for TTS
let content = try await document.extract()

// Get chapters for navigation
for chapter in content.chapters {
    print("\(chapter.title) - Page \(chapter.pageNumber)")
}

// Get text to read aloud
for paragraph in content.paragraphs {
    // Feed to AVSpeechSynthesizer or your TTS engine
    speak(paragraph.text)
}

// Track word positions for highlighting
for word in paragraph.words {
    highlightRect(word.bounds, on: word.pageNumber)
}
```

### Android (Kotlin) - Coming Soon

```kotlin
// Kotlin bindings will provide similar idiomatic API
val document = PDFDocument(file)
val content = document.extract()
```

### Web (JavaScript) - Coming Soon

```javascript
// WASM bindings will provide JavaScript API
const document = await VoxPDF.open("book.pdf");
const content = await document.extract();
```

## Architecture

```
┌─────────────────────────────────────┐
│   Platform Bindings (Swift, etc.)  │  ← Idiomatic APIs
│   - PDFDocument wrapper             │    Memory management
│   - Native error types              │    async/await
├─────────────────────────────────────┤
│         C FFI Boundary              │  ← Stable interface
│   - Opaque pointers                 │    Cross-platform
│   - C structs                       │    Clear ownership
├─────────────────────────────────────┤
│      Rust Core (voxpdf-core)        │  ← All business logic
│   - PDF parsing (lopdf/mupdf)       │    Memory safe
│   - Text extraction                 │    Performance
│   - Layout analysis                 │    Portable
└─────────────────────────────────────┘
```

## Features

### ✅ Phase 0: Foundation (COMPLETE)
- Project structure
- Rust workspace setup
- Documentation (ARCHITECTURE.md, ROADMAP.md)
- Development plan

### 🚧 Next: Slice 0 (Validation Spike)
- Validate lopdf word position extraction
- Decision: Continue with lopdf or pivot to mupdf-sys

### 🎯 v0.1.0 (4-6 weeks) - Better than PDFKit
- Basic text extraction from PDFs
- Word position tracking (for highlighting)
- Paragraph detection
- Hyphenation reassembly
- TOC extraction from PDF metadata
- Swift bindings + Swift Package Manager distribution

### 🎯 v0.5.0 (3-4 months) - Technical Books
- Multi-column layout detection
- Header/footer removal
- Intelligent reading order
- Chapter detection via font analysis

### 🌟 v1.0.0 (9-12 months) - Production Ready
- Complex layouts (3-column, mixed)
- Footnote handling
- Table detection
- Android (Kotlin) bindings
- Comprehensive test suite (100+ PDFs)

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

**Coming Soon** - VoxPDF will be available once v0.1.0 is released.

### Rust (Core Library)

```toml
[dependencies]
voxpdf-core = "0.1"
```

### iOS/macOS (Swift Package Manager)

```swift
dependencies: [
    .package(url: "https://github.com/yourusername/voxpdf-swift.git", from: "0.1.0")
]
```

### Android (Gradle)

```gradle
// Coming in v1.0
implementation 'com.voxpdf:voxpdf-kotlin:0.1.0'
```

### Web (npm)

```bash
# Coming in v1.0+
npm install voxpdf-wasm
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
- **Rust** - Memory-safe systems language enabling cross-platform performance
- **lopdf** - Pure Rust PDF parsing library (starting point)
- **MuPDF** - Industry-standard PDF library (fallback option)
- **Claude Code** - AI pair programming for incremental development
- **Community** - Test PDFs, bug reports, and feature requests from users worldwide

Inspired by:
- **Foxit SDK** - Commercial PDF SDK (C++ core) that proved the compiled core + FFI + bindings architecture
- **PSPDFKit** - Commercial SDK (C++ core) demonstrating cross-platform PDF excellence
- **Voice Dream Reader** - iOS app that showed the importance of precise word highlighting
- **MuPDF** - Open-source PDF library (C core) demonstrating what's achievable
- **Listen2** - Our TTS app that sparked the need for VoxPDF

We follow the same proven pattern as commercial SDKs, but use Rust instead of C++ for memory safety.

## Support

- 🐛 [Report a bug](https://github.com/yourusername/VoxPDF/issues/new?template=bug_report.md)
- 💡 [Request a feature](https://github.com/yourusername/VoxPDF/issues/new?template=feature_request.md)
- 📖 [Submit a problematic PDF](https://github.com/yourusername/VoxPDF/issues/new?template=pdf_test_case.md)
- 💬 [Discussions](https://github.com/yourusername/VoxPDF/discussions)

---

**Status**: 🚧 Phase 0 Complete - Foundation laid, validation spike next
**Architecture**: Rust core + C FFI + platform bindings (Swift, Kotlin, WASM)
**Maintainer**: [@zachswift](https://github.com/zachswift)
**Started**: November 2025
**Next Milestone**: Slice 0 validation spike (2-3 days)
**First Release Target**: February 2026 (v0.1.0)

**Star ⭐ this repo if you believe TTS should be accessible to everyone!**
