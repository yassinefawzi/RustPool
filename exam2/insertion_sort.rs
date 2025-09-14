pub fn insertion_sort(slice: &mut [i32], steps: usize) {
	let len = slice.len();
	for i in 1..len.min(steps + 1) {
		let key = slice[i];
		let mut j = i;
		while j > 0 && slice[j - 1] > key {
			slice[j] = slice[j - 1];
			j -= 1;
		}
		slice[j] = key;
	}
}
