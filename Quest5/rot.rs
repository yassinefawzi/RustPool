pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_uppercase() { b'A' } else { b'a' };
                let pos = (c as u8 - base) as i8;
                let new_pos = (pos + key).rem_euclid(26);
                (base + new_pos as u8) as char
            } else {
                c
            }
        })
        .collect()
}
