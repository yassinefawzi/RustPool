pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
	let mut res = Vec::new();

	for n in 1..=i {
		let spaces =  " ".repeat(n as usize);
		let line = format!("{}{}", spaces, v.repeat(n as usize));
		res.push(line);
	}
	for n in(1..i).rev() {
		let spaces =  " ".repeat(n as usize);
		let line = format!("{}{}", spaces, v.repeat(n as usize));
		res.push(line);
	} 
	res
}