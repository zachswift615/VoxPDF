use crate::models::Paragraph;
use once_cell::sync::Lazy;
use regex::Regex;

// Regex to match hyphenation at end of words
static HYPHEN_PATTERN: Lazy<Regex> = Lazy::new(|| {
    // Matches: word- [space/newline] lowercase-word
    Regex::new(r"(\w+)-\s+([a-z]\w*)").unwrap()
});

pub fn reassemble_hyphenated_words(paragraphs: Vec<Paragraph>) -> Vec<Paragraph> {
    paragraphs.into_iter().map(reassemble_paragraph).collect()
}

fn reassemble_paragraph(para: Paragraph) -> Paragraph {
    let text = para.text;

    // Replace "word- nextpart" with "wordnextpart"
    let reassembled = HYPHEN_PATTERN.replace_all(&text, "$1$2");

    Paragraph::new(
        para.index,
        reassembled.to_string(),
        para.page_number,
        para.words, // TODO: Update word list to match reassembled text
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hyphen_pattern_matches() {
        let text = "exam- ple";
        assert!(HYPHEN_PATTERN.is_match(text));

        let result = HYPHEN_PATTERN.replace_all(text, "$1$2");
        assert_eq!(result, "example");
    }

    #[test]
    fn test_preserves_intentional_hyphens() {
        let text = "self-contained";
        // Should NOT match because both parts start with lowercase
        // Our pattern only matches: word- [space] lowercase
        let result = HYPHEN_PATTERN.replace_all(text, "$1$2");
        assert_eq!(result, "self-contained");
    }
}
