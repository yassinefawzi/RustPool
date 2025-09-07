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
	return true;
}

pub fn prev_prime(nbr: u64) -> u64  {
    if nbr <= 2 {
		return 0;
	}  
	for i in (2..nbr).rev() {
		if is_prime(i) {
			return i;
		}
	}
	0
}