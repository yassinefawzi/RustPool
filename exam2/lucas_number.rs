pub fn lucas_number(n: u32) -> u32 {
	if n == 0 {
		return 2;
	}
	if n == 1 {
		return 1;
	}
	lucas_number(n - 1) + lucas_number(n - 2)
}
