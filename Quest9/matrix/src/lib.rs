#[derive(Debug, Eq, PartialEq)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T> Matrix<T> where T: Default + Copy + From<u8> {
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