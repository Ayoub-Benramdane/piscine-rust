// pub fn arrange_phrase(phrase: &str) -> String {
//     let mut res: Vec<String> = vec![]; 
//     let table: Vec<&str> = phrase.split_whitespace().collect();
//     while res.len() < table.len() {
//         let digit_str = &(res.len() + 1).to_string(); 
//         for i in 0..table.len() {
//             if table[i].contains(digit_str) {
//                 res.push(table[i].replace(digit_str , ""));
//             }
//         }
//     }
//     res.join(" ")
// }

pub fn arrange_phrase(phrase: &str) -> String {
    let mut res: Vec<String> = vec![]; 
    let table: Vec<&str> = phrase.split_whitespace().collect();
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
    res
}