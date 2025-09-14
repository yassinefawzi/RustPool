pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message.is_empty() || message.to_lowercase().contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}
