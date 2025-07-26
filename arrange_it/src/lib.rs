pub fn arrange_phrase(phrase: &str) -> String {
    let mut res : String = String::new();
    let table : Vec<_> = phrase.split(" ").collect();
    let mut count = 1;
    for word in table {
        if word.contains(&count.to_string()) {
            count += 1;
            // res += &(word.chars().filter(|&c| c != count.to_string().chars()).collect() + " ");
        }
    }
    res
}