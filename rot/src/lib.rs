pub fn rotate(input: &str, mut key: i8) -> String {
    let mut res = String::new();
    if key < 0 {
        key = 26 - key.abs();
    }

    for mut c in input.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
            let offset_base = ((c as u8) - (base as u8) + (key as u8)) % 26;
            c = (offset_base + (base as u8)) as char
        }
        res.push(c);
    }
    res
}
