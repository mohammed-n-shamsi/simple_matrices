use super::RVector;

// Implement vector, vector addition override where both inputs are consumed
impl<T> std::ops::Add for RVector<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.valid && other.valid && self.data.len() == other.data.len() {
            let mut new_data = self.data.clone();
            let iter_range = 0..self.data.len();
            for idx in iter_range {
                new_data[idx] = self.data[idx] + other.data[idx];
            }
            RVector {
                data: new_data,
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

// Implement vector, vector addition override where both inputs are references
impl<'a, 'b, T> std::ops::Add<&'b RVector<T>> for &'a RVector<T>
where
    T: std::ops::Add<T, Output = T>,
    T: Copy,
{
    type Output = RVector<T>;
    fn add(self, other: &'b RVector<T>) -> RVector<T> {
        if self.valid && other.valid && self.data.len() == other.data.len() {
            let mut new_data = self.data.to_vec();
            let iter_range = 0..self.data.len();
            for idx in iter_range {
                new_data[idx] = self.data[idx] + other.data[idx];
            }
            RVector {
                data: new_data,
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
#[path = "./tests/rvector_addition_tests.rs"]
mod rvector_addition_tests;
