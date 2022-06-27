use super::RMatrix;
use crate::OpErrors;

// Implement vector, vector dot product override where both inputs are consumed
// returns a vector of size 1
impl<T> std::ops::Mul for RMatrix<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = Result<T, OpErrors>;
    fn mul(self, other: Self) -> Self::Output {}
}

// Implement vector, vector dot product override where neither inputs are consumed
// returns a vector of size 1
impl<'a, 'b, T> std::ops::Mul<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = Result<T, OpErrors>;
    fn mul(self, other: &'b RMatrix<T>) -> Self::Output {}
}

#[cfg(test)]
#[path = "./tests/rmatrix_cross_multiplication_tests.rs"]
mod rmatrix_cross_multiplication_tests;
