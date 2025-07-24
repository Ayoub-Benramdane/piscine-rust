pub fn factorial(num: u64) -> u64 {
    let mut res = 1;
    for i in 1..num+1 {
        res = res * i;
    }
    res
}