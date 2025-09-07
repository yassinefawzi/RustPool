use std::collections::HashMap;
pub fn counting_words(words: &str) -> HashMap<String, u32> {
	let mut counts = HashMap::new();
	let text = words.to_lowercase();

	let cleaned = text.split_whitespace().map(|word| {
		let mut cleaned = String::new();
		let mut prev = false;

		for c in word.chars() {
			if c.is_alphanumeric() {
				cleaned.push(c);
				prev = true;
			} else if c == '\'' && prev {
				cleaned.push(c);
				prev = false;
			} else {
				prev = false;
			}
		}
		cleaned.trim_matches('\'').to_string()
	}).filter(|word| !word.is_empty());

	for word in cleaned {
		*counts.entry(word).or_insert(0) += 1;
	}
	counts
}
