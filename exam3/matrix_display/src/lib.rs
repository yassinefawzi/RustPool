#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut inner = Vec::new();
        for row in slice {
            inner.push(row.to_vec());
        }
        Self(inner)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            write!(f, "(")?;
            for (j, el) in row.iter().enumerate() {
                write!(f, "{}", el)?;
                if j != row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, ")")?;
            if i != self.0.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}
