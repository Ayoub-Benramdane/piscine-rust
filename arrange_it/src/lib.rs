pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec![]; 
    let table: Vec<&str> = phrase.split_whitespace().collect();
    while res.len() < table.len() {
        let digit_str = &(res.len() + 1).to_string(); 
        for i in 0..table.len() {
            if table[i].contains(digit_str) {
                res.push(table[i].replace(digit_str , ""));
            }
        }
    }
    res.join(" ")
}