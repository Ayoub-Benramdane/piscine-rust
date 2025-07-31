pub fn check_ms(message: &str) -> Result<&str, &str> {
    if message == "" || message.contains("stupid") {
      return Err("ERROR: illegal")
    }
  Ok(message)
}