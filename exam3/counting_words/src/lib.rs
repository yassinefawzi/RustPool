use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut hash = HashMap::new();

    for word in words.split_whitespace() {
		let mut cleaned = word.to_lowercase();

		cleaned = cleaned.trim_matches(|c : char| !c.is_alphanumeric() && c!= '\'').to_string();
		if cleaned.is_empty() {
			continue;
		}

		let clean = cleaned.trim_matches('\'').to_string();
		if clean.is_empty() {
			continue;
		}
		*hash.entry(clean).or_insert(0) += 1;
	}
	hash
}
