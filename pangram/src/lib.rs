use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut my_map = HashMap::new();
    
    for c in s.chars() {
        if c.is_ascii_alphabetic() {
            my_map.insert(c, true);
        }
    }
    my_map.len() >= 26
}