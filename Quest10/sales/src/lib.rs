#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<(String, f32)>,
    pub receipt: Vec<f32>,
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: vec![],
            receipt: vec![],
        }
    }

    pub fn insert_item(&mut self, s: &Store, ele: String) {
        let pos = s.products
            .iter()
            .position(|(x, _)| *x == ele)
            .unwrap();
        self.items.push((ele, s.products[pos].1));
    }

    pub fn get_prices(&self) -> Vec<f32> {
        self.items
            .iter()
            .map(|(_, v)| *v)
            .collect::<Vec<f32>>()
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        let mut price = self.get_prices();
        let cal = self.items.len() / 3;
        price.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let v: Vec<f32> = price[cal..].to_vec();

        let prs: f32 =
            ((v.iter().sum::<f32>() * 100.0) as f32) / (price.iter().sum::<f32>() as f32);

        self.receipt = price
            .iter()
            .map(|price| round_two((price * prs) / 100.0))
            .collect::<Vec<f32>>();

        self.receipt.clone()
    }
}

fn round_two(nbr: f32) -> f32 {
    (nbr * 100.0).round() / 100.0
}
