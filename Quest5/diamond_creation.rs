pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as u8 - b'A';
    let mut lines = Vec::new();
    for i in 0..=n {
        let letter = ('A' as u8 + i) as char;
        let leading_spaces = n - i;
        let middle_spaces = if i == 0 { 0 } else { 2 * i - 1 };

        let line = if middle_spaces == 0 {
            format!(
                "{}{}{}",
                " ".repeat(leading_spaces as usize),
                letter,
                " ".repeat(leading_spaces as usize)
            )
        } else {
            format!(
                "{}{}{}{}{}",
                " ".repeat(leading_spaces as usize),
                letter,
                " ".repeat(middle_spaces as usize),
                letter,
                " ".repeat(leading_spaces as usize),
            )
        };
        lines.push(line);
    }
    for i in (0..n).rev() {
        lines.push(lines[i as usize].clone());
    }
    lines
}
