pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let nc: f64 = c as f64;
    (c, nc.exp(), nc.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let mut new_a : String = String::new();
    let nb : Vec<&str> = a.split(" ").collect();
    for n in nb {
        let res : f64 = n.parse().unwrap();
        new_a += &(res.exp().to_string() + " ");
    }
    (a, new_a.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let mut new_vec : Vec<f64> = Vec::new();
    for nb in &b {
        let v : f64 = *nb as f64;
        new_vec.push(v.abs().ln())
    }
    (b, new_vec)
}