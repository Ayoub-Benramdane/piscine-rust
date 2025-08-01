#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut new_str = String::new();
    for ch in original.chars() {
        if ch.is_ascii_uppercase() {
            let jdid = 90 - ((ch as u8) - 65);
            new_str.push(jdid as char);
        } else if ch.is_ascii_lowercase() {
            let jdid = 122 - ((ch as u8) - 97);
            new_str.push(jdid as char);
        } else {
            new_str.push(ch);
        }
    }
    if new_str == ciphered {
        return Ok(());
    }
    Err(CipherError { expected: new_str })
}
