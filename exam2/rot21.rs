pub fn rot21(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                let base = b'a';
                let pos = c as u8 - base;
                ((base + (pos + 21) % 26) as char)
            } else if c.is_ascii_uppercase() {
                let base = b'A';
                let pos = c as u8 - base;
                ((base + (pos + 21) % 26) as char)
            } else {
                c
            }
        })
        .collect()
}
