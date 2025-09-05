pub fn is_empty(v: &str) -> bool {
	return v.len() == 0;
}

pub fn is_ascii(v: &str) -> bool {
	v.chars().all(|c| c.is_ascii())
}

pub fn contains(v: &str, pat: &str) -> bool {
	return v.contains(pat);
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
	return v.split_at(index);
}
	
pub fn find(v: &str, pat: char) -> usize {
	return v.chars().position(|c| c == pat).unwrap_or(v.len())
}
