pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| word.chars().find(|c| c.is_digit(10)).unwrap().to_digit(10).unwrap());
	return words.into_iter()
	.map(|w| w.chars().filter(|c| !c.is_digit(10)).collect::<String>())
	.collect::<Vec<_>>()
	.join(" ");
}
