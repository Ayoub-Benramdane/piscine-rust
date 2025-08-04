pub fn num_to_ordinal(x: u32) -> String {
    let mut nb: Vec<char> = x.to_string().chars().collect();
    let last = nb.pop().unwrap();
    match last  {
        '1' if x % 100 != 11 => {
            let mut res = x.to_string();
            res.push_str("st");
            return res
        }
        '2' if x % 100 != 12 => {
            let mut res = x.to_string();
            res.push_str("nd");
            return res
        }
        '3' if x % 100 != 13 => {
            let mut res = x.to_string();
            res.push_str("rd");
            return res
        }
        _ =>{}
    };
    let mut res = x.to_string();
    res.push_str("th");
    res
}