use super::RMatrix;
use crate::OpErrors;

// Implement vector, vector addition override where both inputs are consumed
impl<T> std::ops::Sub for RMatrix<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {}
}

// Implement vector, vector addition override where both inputs are references
impl<'a, 'b, T> std::ops::Sub<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Sub<T, Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn sub(self, other: &'b RMatrix<T>) -> RMatrix<T> {}
}

#[cfg(test)]
#[path = "./tests/rmatrix_subtraction_tests.rs"]
mod rmatrix_subtraction_tests;
