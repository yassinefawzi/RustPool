#[derive(Debug, PartialEq, Eq)]

pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn transpose(m: Matrix) -> Matrix {
    let Matrix((a, b), (c, d)) = m;
    return	Matrix((a, c), (b, d));
}
