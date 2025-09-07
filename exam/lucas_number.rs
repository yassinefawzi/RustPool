pub fn lucas_number(n: u32) -> u32 {
	if n == 0 {
		return 2;
	} else if n == 1 {
		return 1;
	}
	let mut prev = 2;
	let mut a = 1;
	for _ in 2..=n {
		let c = a + prev;
		prev = a;
		a = c;
	}
	return a;
}

