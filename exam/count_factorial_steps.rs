pub fn count_factorial_steps(factorial: u64) -> u64 {
	if factorial == 1 || factorial == 0 {
		return 0;
	}
	let mut num = 1;
	for i in 1.. {
		num *= i;
		if num == factorial {
			return i;
		} else if num >= factorial {
			return 0;
		}
	}
	return 0;
}