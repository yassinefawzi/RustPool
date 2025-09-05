pub fn nbr_function(c: i32) -> (i32, f64, f64) {
	return (c, (c as f64).exp(), (c.abs() as f64).ln())
}

pub fn str_function(a: String) -> (String, String) {
	let xp = a.split_whitespace().map(|x| x.parse::<i32>().unwrap())
	.map(|x| (x as f64).exp()).map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

	return (a, xp);
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
	let ln = b.iter().map(|n| (n.abs() as f64).ln()).collect::<Vec<f64>>();
	return (b, ln)
}