fn is_prime(n: u64) -> bool {
    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn next_prime(nbr: u64) -> u64 {
	if nbr <= 2 {
		return 2;
    }
	let mut n = nbr;
    loop {
        if is_prime(n) {
			break;
        }
		n += 1;
    }
    n
}
