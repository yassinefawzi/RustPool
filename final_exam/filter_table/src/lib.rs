#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}

impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }

    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }

    pub fn filter_col(&self, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let mut new_h = Vec::new();
        let mut indices = Vec::new();
        for (i, header) in self.headers.iter().enumerate() {
            if filter(header) {
                new_h.push(header.clone());
                indices.push(i);
            }
        }

        if new_h.is_empty() {
            return None;
        }
        let mut new_b = Vec::new();
        for row in &self.body {
            let new_row: Vec<String> = indices
                .iter()
                .map(|&i| row[i].clone())
                .collect();
            new_b.push(new_row);
        }

        Some(Table {
            headers: new_h,
            body: new_b,
        })
    }

    pub fn filter_row(&self, col_name: &str, filter: impl Fn(&str) -> bool) -> Option<Self> {
        let col_index = self.headers.iter().position(|h| h == col_name)?;
        let new_body: Vec<Vec<String>> = self.body
            .iter()
            .filter(|row| filter(&row[col_index]))
            .cloned()
            .collect();

        Some(Table {
            headers: self.headers.clone(),
            body: new_body,
        })
    }
}
