pub fn reverse_it(v: i32) -> String {
	let mut r : i64 = v as i64;
	if v < 0 {
		r *= -1;
	}
	if v < 0 {
		return format!("-{}{}", r.to_string().chars().rev().collect::<String>(), r.to_string());
	}
	format!("{}{}", r.to_string().chars().rev().collect::<String>(), r.to_string())
}