pub fn parts_sums(arr: &[u64]) -> Vec<u64>{
	let mut res = Vec::new();

	for i in (0..arr.len()).rev(){
		let mut holder = 0;
		for j in 0..=i {
			holder += arr[j];
		}
		res.push(holder);
	}
	res.push(0);
	return res;
}
