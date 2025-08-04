pub fn pig_latin(text: &str) -> String {
    let mut res = String::new();
    if text.starts_with('a') || text.starts_with('e') || text.starts_with('i') || text.starts_with('o') || text.starts_with('u') {
        res.push_str(&format!("{}ay", text));
    } else if text[1..=2] == "qu".to_string() {
        res.push_str(&format!("{}{}ay", &text[3..], &text[0..3]));
    } else {
        for (i, c) in text.chars().enumerate() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                res.push_str(&format!("{}{}ay", &text[i..], &text[0..i]));
                break;
            }
        }
    }
    res
}