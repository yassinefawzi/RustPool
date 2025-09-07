pub fn parts_sums(v: &[i32]) -> Vec<i32> {
	let mut result = Vec::new();

	for i in 0..=v.len() {
		let mut sum = 0;
		for j in 0..v.len() - i {
			sum += v[j];
		}
		result.push(sum);
	}
	return result;
}