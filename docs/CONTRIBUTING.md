# Contributing to VoxPDF

Thank you for your interest in contributing to VoxPDF! This document provides guidelines and information for contributors.

---

## Mission

VoxPDF exists to make PDF-to-speech accessible to everyone through open-source technology. Every contribution helps achieve this mission.

---

## Ways to Contribute

### 🐛 Report Bugs

Found a PDF that VoxPDF handles poorly? Help us fix it!

**How to report**:
1. Create a [Bug Report issue](https://github.com/user/voxpdf/issues/new?template=bug_report.md)
2. Include:
   - PDF that demonstrates the problem (if shareable)
   - Expected behavior
   - Actual behavior
   - VoxPDF version
   - Platform (iOS/Android/etc.)

**Can't share the PDF?** Describe the issue in detail:
- PDF generator (LaTeX, Word, InDesign, etc.)
- Layout type (single column, two column, complex)
- What went wrong (wrong reading order, missing text, etc.)

### 📚 Submit Test PDFs

We need a diverse test suite! Submit PDFs that:
- Use different generators
- Have different layouts
- Represent real-world documents
- Have known-correct extraction results

[Submit a Test PDF](https://github.com/user/voxpdf/issues/new?template=pdf_test_case.md)

### 💡 Request Features

Have an idea? [Create a Feature Request](https://github.com/user/voxpdf/issues/new?template=feature_request.md)

**Good feature requests include**:
- Clear use case
- Why it matters for TTS
- How it helps accessibility

### 📝 Improve Documentation

Documentation helps everyone:
- Fix typos or unclear explanations
- Add examples
- Improve API docs
- Write tutorials

### 💻 Contribute Code

See "Development Setup" below for getting started.

---

## Development Setup

### Prerequisites

**Required**:
- Rust 1.70+ (`rustup install stable`)
- Git

**For iOS development**:
- Xcode 15+
- macOS 13+

**For Android development**:
- Android Studio
- NDK 25+

### Cloning the Repository

```bash
git clone https://github.com/user/voxpdf.git
cd voxpdf
```

### Building the Rust Core

```bash
cd voxpdf-core
cargo build
cargo test
```

### Building iOS Bindings

```bash
cd voxpdf-swift
swift build
swift test
```

### Running Tests

```bash
# Rust tests
cd voxpdf-core
cargo test

# Swift tests
cd voxpdf-swift
swift test

# Integration tests
./scripts/run-integration-tests.sh
```

---

## Project Structure

```
voxpdf/
├── voxpdf-core/          # Rust library (core logic)
│   ├── src/
│   │   ├── lib.rs
│   │   ├── ffi.rs
│   │   └── ...
│   ├── tests/
│   └── Cargo.toml
│
├── voxpdf-swift/         # iOS/macOS bindings
│   ├── Sources/VoxPDF/
│   ├── Tests/
│   └── Package.swift
│
├── voxpdf-kotlin/        # Android bindings (future)
├── voxpdf-wasm/          # Web bindings (future)
│
├── docs/                 # Documentation
├── examples/             # Example apps
└── scripts/              # Build scripts
```

---

## Contribution Workflow

### 1. Find an Issue

**Good first issues**: [`good-first-issue` label](https://github.com/user/voxpdf/labels/good-first-issue)

**Help wanted**: [`help-wanted` label](https://github.com/user/voxpdf/labels/help-wanted)

Or create your own issue first to discuss the approach.

### 2. Fork and Branch

```bash
# Fork on GitHub, then:
git clone https://github.com/YOUR_USERNAME/voxpdf.git
cd voxpdf
git checkout -b feature/your-feature-name
```

### 3. Make Changes

- Write code following style guidelines (below)
- Add tests for new functionality
- Update documentation
- Ensure all tests pass

### 4. Commit

```bash
git add .
git commit -m "feat: add column detection for 3-column layouts"
```

**Commit message format**:
- `feat:` New feature
- `fix:` Bug fix
- `docs:` Documentation changes
- `test:` Test additions/changes
- `refactor:` Code refactoring
- `perf:` Performance improvements

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

**PR checklist**:
- [ ] Tests pass locally
- [ ] Added tests for new code
- [ ] Updated documentation
- [ ] Followed code style
- [ ] Clear commit messages

---

## Code Style

### Rust

Use `rustfmt` and `clippy`:

```bash
cargo fmt
cargo clippy -- -D warnings
```

**Guidelines**:
- Prefer explicit types for public APIs
- Document public functions with `///` comments
- Use descriptive variable names
- Avoid `unwrap()` in library code (use `?` or handle errors)

**Example**:
```rust
/// Extracts text from a PDF page.
///
/// # Arguments
/// * `page_index` - Zero-based page number
///
/// # Returns
/// Extracted text or error if page doesn't exist
///
/// # Example
/// ```
/// let text = extractor.extract_page(0)?;
/// ```
pub fn extract_page(&self, page_index: usize) -> Result<String> {
    // ...
}
```

### Swift

Use SwiftFormat (config in repo):

```bash
swiftformat Sources/
```

**Guidelines**:
- Follow Swift API Design Guidelines
- Use `async/await` for async operations
- Prefer value types (struct) over classes when appropriate
- Document public APIs with `///` comments

### Testing

**Test Coverage**: Aim for >80% for new code

**Test Structure**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_specific_behavior() {
        // Arrange
        let input = create_test_input();

        // Act
        let result = function_under_test(input);

        // Assert
        assert_eq!(result, expected);
    }
}
```

**Integration Tests**:
- Place in `tests/` directory
- Use real PDFs from `test-pdfs/`
- Test end-to-end behavior

---

## Adding Test PDFs

Test PDFs help ensure VoxPDF works across different PDF generators and layouts.

**Adding a test PDF**:

1. Create a PDF that demonstrates specific behavior
2. Add to `test-pdfs/` with descriptive name:
   ```
   test-pdfs/
   ├── two-column-latex.pdf
   ├── single-column-word.pdf
   └── complex-magazine.pdf
   ```

3. Create corresponding `.expected.json` with correct extraction:
   ```json
   {
     "paragraphs": [
       {"text": "Expected paragraph 1", "page": 0},
       {"text": "Expected paragraph 2", "page": 0}
     ],
     "chapters": [
       {"title": "Chapter 1", "page": 0}
     ]
   }
   ```

4. Add test case referencing the PDF

**Test PDF guidelines**:
- Keep files small (<1MB if possible)
- Use public domain or CC0 content
- No copyrighted content
- Describe the layout type in filename

---

## Performance Guidelines

When contributing performance improvements:

1. **Measure first**: Profile to identify actual bottlenecks
2. **Benchmark**: Use `cargo bench` or equivalent
3. **Document trade-offs**: Speed vs memory vs complexity
4. **Test at scale**: Ensure improvement works on large PDFs (500+ pages)

**Before**:
```rust
// Profile shows this is slow
for word in words {
    positions.push(calculate_position(word));
}
```

**After**:
```rust
// 2x faster with parallel iterator
let positions: Vec<_> = words
    .par_iter()
    .map(calculate_position)
    .collect();
```

---

## Release Process

VoxPDF follows semantic versioning:

- **Major (1.0.0)**: Breaking API changes
- **Minor (0.1.0)**: New features, backward compatible
- **Patch (0.0.1)**: Bug fixes

**Release checklist** (maintainers):
1. Update CHANGELOG.md
2. Bump version in Cargo.toml
3. Run full test suite
4. Create git tag
5. Publish to crates.io
6. Update GitHub release
7. Update documentation

---

## Community Guidelines

### Code of Conduct

Be respectful and inclusive:
- Welcome newcomers
- Respect different skill levels
- Constructive criticism only
- Focus on technical merit
- No harassment or discrimination

### Communication

- **GitHub Issues**: Bug reports, feature requests
- **Pull Requests**: Code contributions
- **Discussions**: Questions, ideas, general chat

### Getting Help

Stuck? Ask for help:
- Comment on your PR
- Create a Discussion
- Ping maintainers (we're friendly!)

---

## Recognition

Contributors are recognized in:
- CONTRIBUTORS.md
- Release notes
- Project README

Significant contributors may be invited as maintainers.

---

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

---

## Questions?

- Open a [Discussion](https://github.com/user/voxpdf/discussions)
- Comment on relevant issue
- Email: voxpdf@example.com

**Thank you for contributing to VoxPDF!** 🚀

Together, we're making PDF-to-speech accessible to everyone.
