pub fn is_prime(n: u64) -> bool {
	if n < 2 {
		return false;
	}
	let x = (n as f64).sqrt() as u64;
	for i in 2..=x {
		if n % i == 0 {
			return false;
		}
	}
	true
}

pub fn next_prime(nbr: u64) -> u64 {
	let mut n = nbr;
	if n <= 2 {
		return 2;
	}
	if n % 2 == 0 {
		n += 1;
	}
    loop {
        if is_prime(n) {
            return n;
        }
        n += 2;
    }
}
