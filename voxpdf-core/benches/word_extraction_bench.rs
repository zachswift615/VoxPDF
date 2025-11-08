use criterion::{black_box, criterion_group, criterion_main, Criterion};
use voxpdf_core::{extraction::extract_word_positions, PDFDocument};

fn bench_word_extraction(c: &mut Criterion) {
    let doc = PDFDocument::open("tests/fixtures/simple.pdf").unwrap();

    c.bench_function("extract_word_positions", |b| {
        b.iter(|| extract_word_positions(black_box(&doc), black_box(0)).unwrap());
    });
}

criterion_group!(benches, bench_word_extraction);
criterion_main!(benches);
