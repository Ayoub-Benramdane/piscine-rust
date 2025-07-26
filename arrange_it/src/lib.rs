pub fn arrange_phrase(phrase: &str) -> String {
    let table: Vec<&str> = phrase.split_whitespace().collect();
    let mut res = String::new();
    let mut count = 1;

    while count <= table.len() {
        let target = count.to_string();
        for &word in &table {
            if word.contains(&target) {
                for ch in word.chars() {
                    if ch.to_string() != target {
                        res.push(ch);
                    }
                }
                res.push(' ');
                break;
            }
        }
        count += 1;
    }
    res.trim_end().to_string()
}