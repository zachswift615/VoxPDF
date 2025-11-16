use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_bounding_box(c: &mut Criterion) {
    let chars: Vec<(char, f32, f32, f32)> = vec![
        ('H', 10.0, 100.0, 12.0),
        ('e', 15.0, 100.0, 12.0),
        ('l', 20.0, 100.0, 12.0),
        ('l', 25.0, 100.0, 12.0),
        ('o', 30.0, 100.0, 12.0),
        (' ', 35.0, 100.0, 12.0),
        ('W', 40.0, 100.0, 12.0),
        ('o', 45.0, 100.0, 12.0),
        ('r', 50.0, 100.0, 12.0),
        ('l', 55.0, 100.0, 12.0),
        ('d', 60.0, 100.0, 12.0),
    ];

    c.bench_function("calculate_bounding_box", |b| {
        b.iter(|| {
            // Simulate the optimized single-pass calculation
            let chars = black_box(&chars);
            let mut min_x = f32::INFINITY;
            let mut min_y = f32::INFINITY;
            let mut max_x = f32::NEG_INFINITY;
            let mut max_y = f32::NEG_INFINITY;
            let mut font_size_sum = 0.0;

            for &(_, x, y, size) in chars {
                min_x = min_x.min(x);
                min_y = min_y.min(y);
                max_x = max_x.max(x + size * 0.6);
                max_y = max_y.max(y + size);
                font_size_sum += size;
            }

            let width = max_x - min_x;
            let height = max_y - min_y;
            let avg_font_size = font_size_sum / chars.len() as f32;

            black_box((min_x, min_y, width, height, avg_font_size))
        });
    });
}

criterion_group!(benches, benchmark_bounding_box);
criterion_main!(benches);
