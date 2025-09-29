fn is_prime(nbr: u64) -> bool {
	if nbr < 2 {
		return false;
	}
	for i in 2..nbr {
		if nbr % i == 0 {
			return false;
		}
	}
	true
}

pub fn prev_prime(nbr: u64) -> u64  {
    if nbr < 2 {
		return 0;
	}
	for i in (2..nbr).rev() {
		if is_prime(i) {
			return i;
		}
	}
	0
}