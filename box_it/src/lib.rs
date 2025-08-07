pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut bx = Vec::<Box<u32>>::new();
    let values = s.trim().split_whitespace().collect::<Vec<&str>>();
    for part in values {
        let mut k = false;
        if !part.ends_with('k') && !part.chars().all(char::is_numeric) {
            panic!("Invalid input: {}", part);
        } else {
            let number_str = if part.ends_with('k') {
                k = true;
                &part[..part.len() - 1]
            } else {
                part
            };
            let mut number: f64 = number_str.parse::<f64>().expect("Failed to parse number");
            if k {
                number *= 1000.0;
            }
            bx.push(Box::new(number as u32));
        }
    };
    bx
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut unboxed = Vec::<u32>::new();
    for b in 0..a.len() {
            unboxed.push(*a[b]);
    }
    unboxed
}