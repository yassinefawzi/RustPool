fn is_prime(n: u64) -> bool {
	for i in 2..=n/2 {
		if n % i == 0 {
			return false;
		}
	}
	true
}

pub fn prev_prime(nbr: u64) -> u64  {
    if nbr <= 2 {
		return 0;
	}
	for i in (0..nbr).rev(){
		if is_prime(i) {
			return i;
		}
	}
	0
}