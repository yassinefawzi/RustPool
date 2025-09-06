use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
	let mut result = HashMap::new();
	for &word in words {
		*result.entry(word).or_insert(0) += 1;
	}
	return result;
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
	return frequency_count.len();
}

