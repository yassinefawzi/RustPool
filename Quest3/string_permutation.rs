pub fn is_permutation(s1: &str, s2: &str) -> bool {
	let mut str = s1.chars().collect::<Vec<char>>();
	let mut str2 = s2.chars().collect::<Vec<char>>();

	str.sort_unstable();
	str2.sort_unstable();
	return str == str2;
}