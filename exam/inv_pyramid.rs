pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
	let mut result = Vec::new();

	for i in 1..=i {
		let spaces = " ".repeat(i as usize);
		let stars = v.repeat(i as usize);
		result.push(format!("{}{}", spaces, stars));
	}
	for i in (1..i).rev() {
		let spaces = " ".repeat(i as usize);
		let stars = v.repeat((i) as usize);
		result.push(format!("{}{}", spaces, stars));
	}
	return result;
}