use super::RMatrix;
use crate::OpErrors;

// Implement vector, vector addition override where both inputs are consumed
impl<T> std::ops::Add for RMatrix<T>
where
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {}
}

// Implement vector, vector addition override where both inputs are references
impl<'a, 'b, T> std::ops::Add<&'b RMatrix<T>> for &'a RMatrix<T>
where
    T: std::ops::Add<T, Output = T>,
    T: Copy,
{
    type Output = RMatrix<T>;
    fn add(self, other: &'b RMatrix<T>) -> RMatrix<T> {}
}

#[cfg(test)]
#[path = "./tests/rmatrix_addition_tests.rs"]
mod rmatrix_addition_tests;
