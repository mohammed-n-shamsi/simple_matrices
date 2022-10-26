use super::RMatrix;
use super::RMatrixDetails;
use crate::OpErrors;

// Implement vector, vector addition override where both inputs are consumed
impl<T> std::ops::Sub for RMatrix<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

// Implement vector, vector addition override where both inputs are references
impl<'a, 'b, T> std::ops::Sub<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Sub<T, Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn sub(self, other: &'b RMatrix<T>) -> RMatrix<T> {
        if let (Ok(my_mat), Ok(other_mat)) = (self.matrix, other.matrix) {
            if (my_mat.rows == other_mat.rows) && (my_mat.cols == other_mat.cols) {
                let mut new_matrix = Vec::new();
                for row in 0..my_mat.rows {
                    let mut new_col = Vec::new();
                    for col in 0..my_mat.cols {
                        new_col.push(my_mat.data[row][col] - other_mat.data[row][col]);
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
#[path = "./tests/rmatrix_subtraction_tests.rs"]
mod rmatrix_subtraction_tests;
