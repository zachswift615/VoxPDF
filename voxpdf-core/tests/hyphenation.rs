use voxpdf_core::extraction::reassemble_hyphenated_words;
use voxpdf_core::models::Paragraph;

#[test]
fn test_reassemble_simple_hyphenation() {
    // Create paragraph with hyphenated word
    let para = Paragraph::new(
        0,
        "This is an exam- ple of hyphenation".to_string(),
        0,
        vec![]
    );

    let result = reassemble_hyphenated_words(vec![para]);

    assert_eq!(result.len(), 1);
    assert!(result[0].text.contains("example"));
    assert!(!result[0].text.contains("exam-"));
}

#[test]
fn test_preserve_intentional_hyphens() {
    let para = Paragraph::new(
        0,
        "This is a self-contained example".to_string(),
        0,
        vec![]
    );

    let result = reassemble_hyphenated_words(vec![para]);

    // Should preserve intentional hyphen
    assert!(result[0].text.contains("self-contained"));
}
