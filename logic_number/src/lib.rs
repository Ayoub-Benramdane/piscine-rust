pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let mut new_num: u32 = 0;
    let mut res: u32 = 0;
    for c in num_str.chars() {
        new_num = c.to_string().parse::<u32>().unwrap();
        res += new_num.pow(num_str.len() as u32)
    }
    res == num
}