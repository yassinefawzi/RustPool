use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
	let mut h = HashMap::new();
	let l = words.to_lowercase();
    let filt = l.split(|c: char| !c.is_alphanumeric() && c != '\'').map(|word| word.trim_matches('\'')).
	filter(|word| !word.is_empty());
	for word in filt {
		*h.entry(word.to_string()).or_insert(0) += 1;
	}
	h
}