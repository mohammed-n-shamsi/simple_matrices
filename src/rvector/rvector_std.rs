use super::RVector;
use crate::OpErrors;

const CROSS_PRODUCT_SIZE: usize = 3;

impl<T> RVector<T>
where
    T: Copy,
    T: std::ops::Mul<Output = T>,
    T: std::ops::Div<Output = T>,
    T: std::ops::Add<Output = T>,
    T: std::ops::Sub<Output = T>,
{
    pub fn new(data: Vec<T>) -> Self {
        RVector { data: Ok(data) }
    }

    // Return the magnitude squared, squaring will take away from
    // the generic-ness of the data, let caller handle square rooting
    pub fn magnitude_squared(&self) -> Result<T, OpErrors> {
        self.dot_product(self)
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

    pub fn dot_product(&self, other: &RVector<T>) -> Result<T, OpErrors> {
        if let (Ok(my_data), Ok(other_data)) = (self.data.as_ref(), other.data.as_ref()) {
            if my_data.len() == other_data.len() {
                let mut new_data = my_data[0] * other_data[0];
                let iter_range = 1..my_data.len();
                for idx in iter_range {
                    new_data = new_data + my_data[idx] * other_data[idx];
                }
                Ok(new_data)
            } else {
                Err(OpErrors::MismatchedSizes)
            }
        } else {
            Err(OpErrors::InvalidInputs)
        }
    }
    pub fn cross_product(&self, other: &RVector<T>) -> RVector<T> {
        if let (Ok(my_data), Ok(other_data)) = (self.data.as_ref(), other.data.as_ref()) {
            if (my_data.len() == CROSS_PRODUCT_SIZE) && (other_data.len() == CROSS_PRODUCT_SIZE) {
                let new_data = vec![
                    my_data[1] * other_data[2] - my_data[2] * other_data[1],
                    my_data[2] * other_data[0] - my_data[0] * other_data[2],
                    my_data[0] * other_data[1] - my_data[1] * other_data[0],
                ];
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
#[path = "./tests/rvector_std_tests.rs"]
mod rvector_std_tests;
