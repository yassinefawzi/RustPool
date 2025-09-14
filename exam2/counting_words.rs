use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut counts = HashMap::new();

    for word in words.split_whitespace() {
        let cleaned: String = word
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '\'')
            .collect();

        // Remove leading/trailing apostrophes and convert to lowercase
        let cleaned = cleaned.trim_matches('\'').to_lowercase();

        if !cleaned.is_empty() {
            *counts.entry(cleaned).or_insert(0) += 1;
        }
    }

    counts
}
