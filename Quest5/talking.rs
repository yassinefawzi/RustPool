pub fn talking(text: &str) -> &str {
    if text.trim().is_empty() {
        return "Just say something!";
    }

    let is_question = text.trim().ends_with('?');
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters && text.chars().all(|c| !c.is_alphabetic() || c.is_uppercase());
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        _ => "Interesting",
    }
}
