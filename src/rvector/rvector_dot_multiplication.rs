use super::RVector;

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
        if self.valid && other.valid && self.data.len() == other.data.len() {
            let mut new_data: T = self.data[0] * other.data[0];
            let iter_range = 1..self.data.len();
            for idx in iter_range {
                new_data = new_data + self.data[idx] * other.data[idx];
            }
            RVector {
                data: vec![new_data],
                valid: true,
            }
        } else {
            RVector {
                data: Vec::new(),
                valid: false,
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
        if self.valid && other.valid && self.data.len() == other.data.len() {
            let mut new_data: T = self.data[0] * other.data[0];
            let iter_range = 1..self.data.len();
            for idx in iter_range {
                new_data = new_data + self.data[idx] * other.data[idx];
            }
            RVector {
                data: vec![new_data],
                valid: true,
            }
        } else {
            RVector {
                data: Vec::new(),
                valid: false,
            }
        }
    }
}

#[cfg(test)]
#[path = "./tests/rvector_dot_multiplication_tests.rs"]
mod rvector_dot_multiplication_tests;
