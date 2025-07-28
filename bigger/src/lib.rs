use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut max = i32::MIN;
    for (_, value) in h {
        if value > max {
            max = value
        }
    }
    max
}