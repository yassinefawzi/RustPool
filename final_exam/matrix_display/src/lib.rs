use std::fmt;

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let data = slice.iter().map(|row| row.to_vec()).collect();
		Matrix(data)
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
			write!(f, "(")?;
			for (j, col) in row.iter().enumerate() {
				write!(f, "{}", col)?;
				if j < row.len() -1{
					write!(f, " ")?;
				}
			}
			write!(f, ")")?;
			if i < self.0.len() -1 {
				write!(f, "\n")?;
			}
		}
		Ok(())
    }
}
