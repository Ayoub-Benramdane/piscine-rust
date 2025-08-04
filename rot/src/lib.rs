pub fn rotate(input: &str, mut key: i8) -> String {
    let mut res = String::new();
    if key < 0 {
        key = 26 - key.abs();
    }

    for c in input.chars() {
        if c.is_alphabetic() {
            let base = if c.is_ascii_uppercase() { 'A' } else { 'a' };
            let offset_base = ((c as u8) - (base as u8) + (key as u8)) % 26;
            res.push((offset_base + (base as u8)) as char);
        } else {
            res.push(c);
        }
    }
    res
}
