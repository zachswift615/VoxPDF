use criterion::{black_box, criterion_group, criterion_main, Criterion};
use voxpdf_core::{PDFDocument, extraction::extract_word_positions};

fn benchmark_word_extraction(c: &mut Criterion) {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();

    c.bench_function("extract_words_single_page", |b| {
        b.iter(|| {
            extract_word_positions(&doc, black_box(0))
        });
    });
}

criterion_group!(benches, benchmark_word_extraction);
criterion_main!(benches);
