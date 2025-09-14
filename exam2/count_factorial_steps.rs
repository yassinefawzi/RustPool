pub fn count_factorial_steps(factorial: u64) -> u64 {
	if factorial <= 1 {
		return 0;
	}
	let mut res = 1;
	let mut i: u64 = 1;

	while res < factorial {
		i += 1;
		res *= i;
	}
	if res == factorial{
		return i;
	}
	0
}