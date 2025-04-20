pub fn check_ms(message: &str) -> Result<&str, &str> {
    let blocked_words = ["stupid", "dumb", "idiot", "fool"];

    if message.is_empty() || blocked_words.iter().any(|&word| message.contains(word)) {
        Err("ERROR: illegal")
    } else {
        Ok(message)
    }
}