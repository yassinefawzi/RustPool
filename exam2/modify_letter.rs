pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars()
        .filter(|&c| c.to_ascii_lowercase() != letter && c.to_ascii_uppercase() != letter)
        .collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    s.chars()
        .map(|c| {
            if c.to_ascii_lowercase() == letter.to_ascii_lowercase() {
                if c.is_lowercase() {
                    c.to_ascii_uppercase()
                } else {
                    c.to_ascii_lowercase()
                }
            } else {
                c
            }
        })
        .collect::<String>()
}
