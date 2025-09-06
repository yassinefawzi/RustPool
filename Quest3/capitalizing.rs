pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    return match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    };
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in input.chars() {
        if c.is_whitespace() {
            capitalize_next = true;
            result.push(c);
        } else if capitalize_next {
            result.push_str(&c.to_uppercase().collect::<String>());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
	return result;
}

pub fn change_case(input: &str) -> String {
    return input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().collect::<String>()
            } else if c.is_uppercase() {
                c.to_lowercase().collect::<String>()
            } else {
                c.to_string()
            }
        })
        .collect::<String>();
}
