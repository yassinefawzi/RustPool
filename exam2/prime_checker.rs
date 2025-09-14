#[derive(PartialEq, Eq, Debug)]
pub enum PrimeErr {
    Even,
    Divider(u32),
}

pub fn prime_checker(nb: u32) -> Option<Result<u32, PrimeErr>> {
    if nb <= 1 {
        return None;
    }
    if nb == 2 {
        return Some(Ok(2));
    }
    if nb % 2 == 0 {
        return Some(Err(PrimeErr::Even));
    }

    let sqrt_nb = (nb as f64).sqrt() as u32;
    for i in (3..=sqrt_nb).step_by(2) {
        if nb % i == 0 {
            return Some(Err(PrimeErr::Divider(i)));
        }
    }

    Some(Ok(nb))
}
