pub fn first_subword(mut s: String) -> String {
    let mut count = 0;
    for ch in s.chars() {
        if (ch.is_uppercase() || ch == '_')&& count != 0 {
            return s[0..count].to_string()
        }
        count += 1;
    }
    s
}