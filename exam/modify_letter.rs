pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars()
        .filter(|&c| !c.eq_ignore_ascii_case(&letter))
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() && c == letter {
                c.to_ascii_uppercase()
            } else if c.is_uppercase() && c == letter.to_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        }).collect()
}