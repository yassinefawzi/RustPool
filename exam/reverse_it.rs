pub fn reverse_it(v: i32) -> String {
	if v < 0 {
		let abv = v as i64 * -1;
		let s: String = abv.to_string().chars().rev().collect();
		return format!("-{}{}", s, abv);
	}
	return format!("{}{}", v.to_string().chars().rev().collect::<String>(), v.to_string())
}
