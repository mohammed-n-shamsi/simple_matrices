use super::RVector;
use crate::OpErrors;

impl<T> RVector<T>
where
    T: Copy,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    pub fn new(data: Vec<T>) -> Self {
        RVector {
            data: Ok(data.clone()),
        }
    }

    // Return the magnitude squared, squaring will take away from
    // the generic-ness of the data, let caller handle square rooting
    pub fn magnitude_squared(&self) -> Result<T, OpErrors> {
        self * self
    }

    pub fn element_division(&self, divisor: T) -> RVector<T> {
        if let Ok(valid_data) = &self.data {
            let mut quotient_vector: Vec<T> = Vec::new();
            for dividend in valid_data.iter() {
                quotient_vector.push(*dividend / divisor);
            }
            RVector::new(quotient_vector)
        } else {
            RVector {
                data: Err(OpErrors::InvalidInputs),
            }
        }
    }

    pub fn element_addition(&self, addend: T) -> RVector<T> {
        if let Ok(valid_data) = &self.data {
            let mut addition_vector: Vec<T> = Vec::new();
            for other_addend in valid_data.iter() {
                addition_vector.push(*other_addend + addend);
            }
            RVector::new(addition_vector)
        } else {
            RVector {
                data: Err(OpErrors::InvalidInputs),
            }
        }
    }
    pub fn element_subtraction(&self, subtrahend: T) -> RVector<T> {
        if let Ok(valid_data) = &self.data {
            let mut difference_vector: Vec<T> = Vec::new();
            for minuend in valid_data.iter() {
                difference_vector.push(*minuend - subtrahend);
            }
            RVector::new(difference_vector)
        } else {
            RVector {
                data: Err(OpErrors::InvalidInputs),
            }
        }
    }
    pub fn element_multiplication(&self, multiplier: T) -> RVector<T> {
        if let Ok(valid_data) = &self.data {
            let mut product_vector: Vec<T> = Vec::new();
            for multiplicand in valid_data.iter() {
                product_vector.push(*multiplicand * multiplier);
            }
            RVector::new(product_vector)
        } else {
            RVector {
                data: Err(OpErrors::InvalidInputs),
            }
        }
    }
}
#[cfg(test)]
#[path = "./tests/rvector_tests.rs"]
mod rvector_tests;
