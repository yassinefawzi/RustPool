pub fn reverse_it(v: i32) -> String {
    let original = v.to_string(); // string of the original number
    let abs_str: String = original.trim_start_matches('-').chars().rev().collect();

    if v < 0 {
        format!("-{}{}", abs_str, original.trim_start_matches('-'))
    } else {
        format!("{}{}", abs_str, original)
    }
}
