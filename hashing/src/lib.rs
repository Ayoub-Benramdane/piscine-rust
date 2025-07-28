use std::collections::HashMap;

pub fn mean(list: &[i32]) -> f64 {
    let nb = list.iter().sum::<i32>() as f64;
    nb / (list.len() as f64)
}

pub fn median(list: &[i32]) -> i32 {
    let mut jdid = list.to_owned();
    jdid.sort();
    if jdid.len() % 2 == 0 {
        return (jdid[jdid.len() / 2] + jdid[jdid.len() / 2 - 1]) / 2;
    }
    jdid[jdid.len() / 2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut counts = HashMap::new();

    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }

    let mut mode = 0;
    let mut max_count = 0;
    let mut first = true;

    for (&num, &count) in &counts {
        if first || count > max_count {
            mode = num;
            max_count = count;
            first = false;
        }
    }

    mode
}
