pub fn scytale_decoder(s: String, letters_per_turn: u32) -> Option<String> {
    if s.is_empty() || letters_per_turn == 0 {
        return None;
    }

    let letters_per_turn = letters_per_turn as usize;
    let len = s.len();
    let rows = (len + letters_per_turn - 1) / letters_per_turn;

    let mut decoded = String::new();

    for col in 0..letters_per_turn {
        for row in 0..rows {
            let index = row * letters_per_turn + col;
            if index < len {
                decoded.push(s.chars().nth(index).unwrap());
            }
        }
    }

    Some(decoded)
}
