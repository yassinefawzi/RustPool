pub fn first_subword(mut s: String) -> String {
	for (i, c) in s.chars().enumerate() {
		if c == '_' || (i != 0 && c.is_uppercase()) {
			s.truncate(i);
			break
		}
	}
	return s;
}
