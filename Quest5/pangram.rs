use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new();

    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            letters.insert(c.to_ascii_lowercase());
        }
    }

    letters.len() == 26
}