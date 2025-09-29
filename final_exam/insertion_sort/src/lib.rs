pub fn insertion_sort(slice: &mut [i32], steps: usize) {
	for i in 1..=steps {
		let mut j = i;
		let hold = slice[i];
		while j > 0 && slice[j - 1] > hold {
			slice[j] = slice[j - 1];
			j -= 1;
		}
		slice[j] = hold;
	}
}