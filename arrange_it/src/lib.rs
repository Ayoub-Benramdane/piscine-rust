pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec![];
    let table: Vec<&str> = phrase.split_whitespace().collect();
    let mut i = 0;
    while i < table.len() {
        let digit_str = &(res.len() + 1).to_string();
        if table[i].contains(digit_str) {
            res.push(table[i].replace(digit_str, ""));
            i = 0;
            continue;
        }
        i += 1;
    }
    res.join(" ")
}
