use super::RMatrix;
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
        RMatrix {
            matrix: Err(OpErrors::InvalidInputs),
        }
    }
}

#[cfg(test)]
#[path = "./tests/rmatrix_multiplication_tests.rs"]
mod rmatrix_multiplication_tests;
