use super::RMatrix;
use super::RMatrixDetails;

// Implement vector, vector equality override where both inputs are references
impl<T> std::cmp::PartialEq for RMatrixDetails<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data && self.rows == other.rows && self.cols == other.cols
    }
}

// Implement vector, vector equality override where both inputs are references
impl<T> std::cmp::PartialEq for RMatrix<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let (Ok(my_data), Ok(other_data)) = (self.matrix.as_ref(), other.matrix.as_ref()) {
            my_data == other_data
        } else if let (Err(my_err), Err(oth_err)) = (self.matrix.as_ref(), other.matrix.as_ref()) {
            my_err == oth_err
        } else {
            false
        }
    }
}
//#[cfg(test)]
//#[path = "./tests/rmatrix_equality_tests.rs"]
//mod rmatrix_equality_tests;
