use super::RVector;
use crate::OpErrors;

// Implement vector, vector addition override where both inputs are consumed
impl<T> std::ops::Sub for RVector<T>
where
    T: std::ops::Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        &self - &other
    }
}

// Implement vector, vector addition override where both inputs are references
impl<'a, 'b, T> std::ops::Sub<&'b RVector<T>> for &'a RVector<T>
where
    T: std::ops::Sub<T, Output = T>,
    T: Copy,
{
    type Output = RVector<T>;
    fn sub(self, other: &'b RVector<T>) -> RVector<T> {
        if let (Ok(my_data), Ok(other_data)) = (self.data.as_ref(), other.data.as_ref()) {
            if my_data.len() == other_data.len() {
                let mut new_data = Vec::new();
                let iter_range = 0..my_data.len();
                for idx in iter_range {
                    new_data.push(my_data[idx] - other_data[idx]);
                }
                RVector { data: Ok(new_data) }
            } else {
                RVector {
                    data: Err(OpErrors::MismatchedSizes),
                }
            }
        } else {
            RVector {
                data: Err(OpErrors::InvalidInputs),
            }
        }
    }
}

#[cfg(test)]
#[path = "./tests/rvector_subtraction_tests.rs"]
mod rvector_subtraction_tests;
