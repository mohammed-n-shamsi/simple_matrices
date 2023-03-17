use super::RMatrix;
use super::RMatrixDetails;
use crate::OpErrors;

// Implement matrix, matrix cross produce override where both inputs are consumed
impl<T> std::ops::Mul for RMatrix<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn mul(self, other: Self) -> Self::Output {
        &self * &other
    }
}

// Implement matrix, matrix product override where neither inputs are consumed
impl<'a, 'b, T> std::ops::Mul<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn mul(self, other: &'b RMatrix<T>) -> Self::Output {
        if let (Ok(my_mat), Ok(other_mat)) = (self.matrix.as_ref(), other.matrix.as_ref()) {
            if my_mat.cols == other_mat.rows {
                let mut new_matrix = Vec::new();
                for row in 0..my_mat.rows {
                    let mut new_row = Vec::new();
                    for col in 0..other_mat.cols {
                        let mut summed_val = my_mat.data[row][0] * other_mat.data[0][col];
                        for iter in 1..other_mat.rows {
                            summed_val =
                                summed_val + my_mat.data[row][iter] * other_mat.data[iter][col];
                        }
                        new_row.push(summed_val);
                    }
                    new_matrix.push(new_row);
                }
                RMatrix {
                    matrix: Ok(RMatrixDetails {
                        data: new_matrix,
                        rows: my_mat.rows,
                        cols: other_mat.cols,
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
#[path = "./tests/rmatrix_multiplication_tests.rs"]
mod rmatrix_multiplication_tests;
