#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}
impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}
impl Cart {
    pub fn new() -> Cart {
        Cart { items: vec![], receipt: vec![] }
    }
    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let mut price = 0.0;
        for (i, v) in &s.products {
            if *i == ele {
                price = *v;
            }
        }
        self.items.push((ele, price))
    }
    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut a = vec![];
        for (_, v) in &self.items {
            a.push(*v);
        }
        a.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut b = vec![];
        for aa in &a {
            let j = *aa - (*aa * a[0]) / a.iter().sum::<f32>();
            b.push(format(j));
        }

        self.receipt = b.clone();
        b
    }
}

pub fn format(a: f32) -> f32 {
    let ab = format!("{:.02}", a);
    let c = ab.parse::<f32>().unwrap_or(0.0);
    c
}
