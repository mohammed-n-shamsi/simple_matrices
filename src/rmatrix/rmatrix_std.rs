use super::RMatrix;
use super::RMatrixDetails;
use crate::OpErrors;

impl<T> RMatrix<T>
where
    T: Copy,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let rows = data.len();
        let cols = data[0].len();
        for col_vec in data.iter() {
            if cols != col_vec.len() {
                return RMatrix {
                    matrix: Err(OpErrors::InvalidInputs),
                };
            }
        }
        RMatrix {
            matrix: Ok(RMatrixDetails { data, cols, rows }),
        }
    }
}

#[cfg(test)]
#[path = "./tests/rmatrix_std_tests.rs"]
mod rmatrix_std_tests;
