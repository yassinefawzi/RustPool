pub fn rot21(input: &str) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c.is_ascii_alphabetic() {
            if c.is_uppercase() {
                let h = ((c as u8 - b'A' + 21) % 26) + b'A';
				res.push(h as char);
            } else {
                let h = ((c as u8 - b'a' + 21) % 26) + b'a';
				res.push(h as char);
            }
        } else {
            res.push(c);
        }
    }
	res
}
