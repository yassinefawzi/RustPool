fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

pub fn pig_latin(word: &str) -> String {
    let word = word.to_lowercase();
    let chars: Vec<char> = word.chars().collect();
    if let Some(first) = chars.get(0) {
        if is_vowel(*first) {
            return format!("{}ay", word);
        }
    }
    if chars.len() >= 3 && !is_vowel(chars[0]) && chars[1] == 'q' && chars[2] == 'u' {
        return format!(
            "{}{}ay",
            chars[3..].iter().collect::<String>(),
            &chars[0..3].iter().collect::<String>()
        );
    }
    let first_vowel_index = chars.iter().position(|&c| is_vowel(c)).unwrap_or(0);
    let (head, tail) = chars.split_at(first_vowel_index);
    format!(
        "{}{}ay",
        tail.iter().collect::<String>(),
        head.iter().collect::<String>()
    )
}
