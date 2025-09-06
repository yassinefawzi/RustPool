pub fn sum(a: &[i32]) -> i32 {
	return a.iter().sum();
}

pub fn thirtytwo_tens() -> [i32; 32] {
	let mut arr = [0; 32];

	for i in 0..32 {
		arr[i] = 10;
	}
	return arr;
}
