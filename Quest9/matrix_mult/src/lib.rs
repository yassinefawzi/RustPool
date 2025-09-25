use std::ops::Mul;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> where T: Copy + Default + Mul<Output = T> + std::ops::Add<Output = T> {
    pub fn number_of_rows(&self) -> usize {
        self.0.len()
    }

    pub fn number_of_cols(&self) -> usize {
        if self.0.is_empty() { 0 } else { self.0[0].len() }
    }

    pub fn row(&self, n: usize) -> Vec<T> {
        self.0[n].clone()
    }

    pub fn col(&self, n: usize) -> Vec<T> {
        self.0
            .iter()
            .map(|r| r[n])
            .collect()
    }
}

impl<T> Mul for Matrix<T> where T: Copy + Default + Mul<Output = T> + std::ops::Add<Output = T> {
    type Output = Option<Matrix<T>>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        let m = self.number_of_rows();
        let n = self.number_of_cols();
        let p = rhs.number_of_cols();

        if n != rhs.number_of_rows() {
            return None;
        }

        let mut result = vec![vec![T::default(); p]; m];

        for i in 0..m {
            for j in 0..p {
                let mut sum = T::default();
                for k in 0..n {
                    sum = sum + self.0[i][k] * rhs.0[k][j];
                }
                result[i][j] = sum;
            }
        }

        Some(Matrix(result))
    }
}
