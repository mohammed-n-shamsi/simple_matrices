use super::RVector;
use crate::OpErrors;

// Implement vector, vector dot product override where both inputs are consumed
// returns a vector of size 1
impl<T> std::ops::Mul for RVector<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if let (Ok(my_data), Ok(other_data)) = (self.data, other.data) {
            if my_data.len() == other_data.len() {
                let mut new_data = my_data[0] * other_data[0];
                let iter_range = 1..my_data.len();
                for idx in iter_range {
                    new_data = new_data + my_data[idx] * other_data[idx];
                }
                RVector {
                    data: Ok(vec![new_data]),
                }
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

// Implement vector, vector dot product override where neither inputs are consumed
// returns a vector of size 1
impl<'a, 'b, T> std::ops::Mul<&'b RVector<T>> for &'a RVector<T>
where
    T: std::ops::Mul<Output = T>,
    T: std::ops::Add<Output = T>,
    T: Copy,
{
    type Output = RVector<T>;
    fn mul(self, other: &'b RVector<T>) -> RVector<T> {
        if let (Ok(my_data), Ok(other_data)) = (self.data.as_ref(), other.data.as_ref()) {
            if my_data.len() == other_data.len() {
                let mut new_data = my_data[0] * other_data[0];
                let iter_range = 1..my_data.len();
                for idx in iter_range {
                    new_data = new_data + my_data[idx] * other_data[idx];
                }
                RVector {
                    data: Ok(vec![new_data]),
                }
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
#[path = "./tests/rvector_dot_multiplication_tests.rs"]
mod rvector_dot_multiplication_tests;
