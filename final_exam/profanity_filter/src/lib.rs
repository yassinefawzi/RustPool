pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() {
        return Err("ERROR: illegal");
    }
    for word in message.split_whitespace() {
        if word == "stupid" {
            return Err("ERROR: illegal");
        }
    }
    Ok(message)
}
