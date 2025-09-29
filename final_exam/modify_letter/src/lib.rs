pub fn remove_letter_sensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c != letter).collect()
}

pub fn remove_letter_insensitive(s: &str, letter: char) -> String {
    s.chars().filter(|&c| c.to_ascii_lowercase() != letter.to_ascii_lowercase() || c.to_ascii_uppercase() != letter.to_ascii_uppercase()).collect()
}

pub fn swap_letter_case(s: &str, letter: char) -> String {
    let mut r = String::new();

	for c in s.chars() {
		if c.to_ascii_lowercase() == letter.to_ascii_lowercase() {
			if c.is_lowercase() {
				r.push(c.to_ascii_uppercase());
			} else {
				r.push(c.to_ascii_lowercase());
			}
		} else {
			r.push(c);
		}
	}
	r
}