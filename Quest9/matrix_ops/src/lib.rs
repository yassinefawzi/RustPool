use std::ops::{ Add, Sub };

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> where T: Copy + Default + From<u8> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![vec![T::default()]])
    }

    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        Matrix(vec![vec![T::default(); col]; row])
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut data = vec![vec![T::default(); n]; n];
        for i in 0..n {
            data[i][i] = T::from(1u8);
        }
        Matrix(data)
    }
}

impl<T> Matrix<T> where T: Copy {
    fn same_dimensions(&self, other: &Matrix<T>) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        for (row_self, row_other) in self.0.iter().zip(other.0.iter()) {
            if row_self.len() != row_other.len() {
                return false;
            }
        }
        true
    }
}

impl<T> Add for Matrix<T> where T: Copy + Add<Output = T> {
    type Output = Option<Matrix<T>>;

    fn add(self, rhs: Matrix<T>) -> Self::Output {
        if !self.same_dimensions(&rhs) {
            return None;
        }

        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut result = vec![vec![self.0[0][0]; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = self.0[i][j] + rhs.0[i][j];
            }
        }

        Some(Matrix(result))
    }
}

impl<T> Sub for Matrix<T> where T: Copy + Sub<Output = T> {
    type Output = Option<Matrix<T>>;

    fn sub(self, rhs: Matrix<T>) -> Self::Output {
        if !self.same_dimensions(&rhs) {
            return None;
        }

        let rows = self.0.len();
        let cols = self.0[0].len();
        let mut result = vec![vec![self.0[0][0]; cols]; rows];

        for i in 0..rows {
            for j in 0..cols {
                result[i][j] = self.0[i][j] - rhs.0[i][j];
            }
        }

        Some(Matrix(result))
    }
}
