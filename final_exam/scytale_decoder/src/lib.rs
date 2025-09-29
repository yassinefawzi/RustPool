pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
	if s.is_empty() || letters_per_turn == 0 {
		return None;
	}
	let mut res = String::new();
	let letters_per_turn = letters_per_turn as usize;
	let len = s.len();
	let cols = (len * letters_per_turn - 1) / letters_per_turn;
	for i in 0..letters_per_turn {
		for col in 0..cols {
			let j = letters_per_turn * col + i;
			if j < len {
				res.push(s.chars().nth(j).unwrap());
			}
		}
	}
	Some(res)
}