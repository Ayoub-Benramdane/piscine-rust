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
        let _ = |x: f32| -> f32 { (x * 100.0).round() / 100.0 };

        let mut a = vec![];
        for (_, v) in &self.items {
            a.push(*v);
        }
        a.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut receipt = vec![];

        let i = a[0..a.len() / 3].iter().sum::<f32>() / a.iter().sum::<f32>();

        for val in &a {
            receipt.push(((*val - *val * i) * 100.0).round() / 100.0);
        }
        self.receipt = receipt.clone();
        receipt
    }
}
