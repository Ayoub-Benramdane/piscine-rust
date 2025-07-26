pub fn arrange_phrase(phrase: &str) -> String {
    let words = phrase.split_whitespace();
    let mut result = String::new();

    for i in 1..=9 {
        let digit = (b'0' + i as u8) as char;
        if let Some(word) = words.clone().find(|w| w.contains(digit)) {
            for ch in word.chars() {
                if ch != digit {
                    result.push(ch);
                }
            }
            result.push(' ');
        }
    }

    result.trim_end().to_string()
}