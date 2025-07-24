pub fn rev_str(input: &str) -> String {
    let mut s = String::new();
    for c in input.chars() {
        s = c.to_string() + &s;
    }
    s
}