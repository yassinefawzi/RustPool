#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

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

pub fn prime_checker(nb: u32) -> Option<Result<i32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }
    if is_prime(nb as u64) {
        return Some(Ok(nb as i32));
    }
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }

	for i in 3..=(nb as f64).sqrt() as u32 {
        if nb % i == 0 {
            return Some(Err(PrimeErr::Divider(i)));
        }
    }
    None
}
