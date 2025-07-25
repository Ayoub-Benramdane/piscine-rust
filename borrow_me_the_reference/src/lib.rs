pub fn delete_and_backspace(s: &mut String) {
    let mut new_s: String = String::new();
    let mut count = 0;
    for ch in s.chars() {
        if ch == '+' {
            count += 1;
            continue;
        } else if count != 0 {
            count -= 1;
            continue;
        } else if ch != '-' {
            new_s.push(ch);
        } else {
            new_s.pop();
        }
    }
    *s = new_s;
}

pub fn do_operations(v: &mut [String]) {
    for nb in v {
        if nb.contains("+") {
            let tbl: Vec<_> = nb.split("+").collect();
            let nb1: i32 = tbl[0].parse().unwrap();
            let nb2: i32 = tbl[1].parse().unwrap();
            *nb = (nb1 + nb2).to_string();
        } else if nb.contains("-") {
            let tbl: Vec<_> = nb.split("-").collect();
            let nb1: i32 = tbl[0].parse().unwrap();
            let nb2: i32 = tbl[1].parse().unwrap();
            *nb = (nb1 - nb2).to_string();
        }
    }
}
