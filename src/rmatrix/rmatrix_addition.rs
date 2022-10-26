use super::RMatrix;
use super::RMatrixDetails;
use crate::OpErrors;

// Implement matrix, matrix addition override where both inputs are consumed
impl<T> std::ops::Add for RMatrix<T>
where
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        &self + &other
    }
}

// Implement Matrix, Matrix addition override where both inputs are references
impl<'a, 'b, T> std::ops::Add<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Add<T, Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn add(self, other: &'b RMatrix<T>) -> RMatrix<T> {
        if let (Ok(my_mat), Ok(other_mat)) = (self.matrix, other.matrix) {
            if (my_mat.rows == other_mat.rows) && (my_mat.cols == other_mat.cols) {
                let mut new_matrix = Vec::new();
                for row in 0..my_mat.rows {
                    let mut new_col = Vec::new();
                    for col in 0..my_mat.cols {
                        new_col.push(my_mat.data[row][col] + other_mat.data[row][col]);
                    }
                    new_matrix.push(new_col)
                }
                RMatrix {
                    matrix: Ok(RMatrixDetails {
                        data: new_matrix,
                        rows: my_mat.rows,
                        cols: my_mat.cols,
                    }),
                }
            } else {
                RMatrix {
                    matrix: Err(OpErrors::MismatchedSizes),
                }
            }
        } else {
            RMatrix {
                matrix: Err(OpErrors::InvalidInputs),
            }
        }
    }
}

#[cfg(test)]
#[path = "./tests/rmatrix_addition_tests.rs"]
mod rmatrix_addition_tests;
