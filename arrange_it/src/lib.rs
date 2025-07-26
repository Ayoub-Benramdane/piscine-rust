pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: String = String::new();
    let table: Vec<_> = phrase.split_whitespace().collect();
    let mut count = 1;
    let mut i = 0;
    while i < table.len() {
        let digit = (b'0' + count as u8) as char;
        if table[i].contains(&digit.to_string()) {
            for ch in table[i].chars() {
                if ch != digit {
                    res.push(ch);
                }
            }
            res.push_str(" ");
            count += 1;
            i = 0;
            continue;
        }
        i += 1;
    }
    res.trim().to_string()
}
