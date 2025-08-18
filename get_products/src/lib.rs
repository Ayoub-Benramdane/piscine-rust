pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    if arr.len() <= 1 {
        return res;
    }
    for i in 0..arr.len() {
        let mut nb = 1;
        for j in 0..arr.len() {
            if arr[i] != arr[j] {
                nb *= arr[j];
            }
        }
        res.push(nb);
    }
    res
}
