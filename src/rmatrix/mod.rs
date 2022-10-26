pub mod rmatrix_addition;
pub mod rmatrix_cross_multiplcation;
pub mod rmatrix_equality;
pub mod rmatrix_std;
pub mod rmatrix_subtraction;

#[derive(Debug)]
struct RMatrixDetails<T> {
    data: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
}

#[derive(Debug)]
pub struct RMatrix<T> {
    matrix: Result<RMatrixDetails<T>, crate::OpErrors>,
}
