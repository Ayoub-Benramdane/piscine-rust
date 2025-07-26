pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: String = String::new();
    let table: Vec<_> = phrase.split(" ").collect();
    let mut count = 1;
    let mut i = 0;
    while i < table.len() {
        if table[i].contains(&count.to_string()) {
            for ch in table[i].chars() {
                if ch != count.to_string().chars().next().expect("") {
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
