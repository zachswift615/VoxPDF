use std::env;
use voxpdf_core::{extraction::extract_word_positions, PDFDocument};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pdf_path = &args[1];
    let page_num = args[2].parse::<u32>().unwrap_or(0);

    let doc = PDFDocument::open(pdf_path).expect("Failed to open PDF");
    let words = extract_word_positions(&doc, page_num).expect("Failed to extract words");

    println!("=== Analyzing Line Spacing ===\n");

    // Group words by Y position to identify lines
    const Y_THRESHOLD: f32 = 5.0;
    let mut lines: Vec<Vec<_>> = Vec::new();
    let mut current_line = Vec::new();
    let mut current_y = words[0].bounds.y;

    for word in &words {
        let word_y = word.bounds.y;
        if (word_y - current_y).abs() < Y_THRESHOLD {
            current_line.push(word);
        } else {
            if !current_line.is_empty() {
                lines.push(current_line);
            }
            current_y = word_y;
            current_line = vec![word];
        }
    }
    if !current_line.is_empty() {
        lines.push(current_line);
    }

    println!("Found {} lines\n", lines.len());

    // Analyze spacing between consecutive lines
    println!("Line spacing analysis (first 20 lines):\n");
    for i in 0..lines.len().min(20) {
        let line = &lines[i];
        let y_pos = line[0].bounds.y;
        let height = line[0].bounds.height;
        let text_preview: String = line
            .iter()
            .take(10)
            .map(|w| w.text.as_str())
            .collect::<Vec<_>>()
            .join(" ");

        if i > 0 {
            let prev_y = lines[i - 1][0].bounds.y;
            let spacing = (y_pos - prev_y).abs();
            let ratio = spacing / height;
            println!(
                "Line {}: y={:.1}, height={:.1}, spacing={:.1}, ratio={:.2}x",
                i, y_pos, height, spacing, ratio
            );
            println!(
                "  Text: {}...\n",
                &text_preview[..text_preview.len().min(60)]
            );
        } else {
            println!("Line {}: y={:.1}, height={:.1}", i, y_pos, height);
            println!(
                "  Text: {}...\n",
                &text_preview[..text_preview.len().min(60)]
            );
        }
    }
}
