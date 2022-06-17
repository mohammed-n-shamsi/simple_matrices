use super::RVector;

// Implement vector, vector equality override where both inputs are references
impl<T> std::cmp::PartialEq for RVector<T>
where
    T: std::cmp::PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if let (Ok(my_data), Ok(other_data)) = (self.data.as_ref(), other.data.as_ref()) {
            my_data == other_data
        } else if let (Err(my_err), Err(other_err)) = (self.data.as_ref(), other.data.as_ref()) {
            my_err == other_err
        } else {
            false
        }
    }
}

#[cfg(test)]
#[path = "./tests/rvector_equality_tests.rs"]
mod rvector_equality_tests;
