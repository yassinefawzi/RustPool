pub fn count_factorial_steps(factorial: u64) -> u64 {
	if factorial == 0 || factorial == 1 {
		return 0;
	}
    let mut i = 1;
	let mut curr = 1;
	loop {
		curr *= i;
		if curr == factorial {
			return i;
		} else if curr > factorial {
			return 0;
		}
		i += 1;
	}
}
