pub mod rmatrix_equality;
pub mod rmatrix_std;

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
