use super::RVector;

impl<T> RVector<T>
where
    T: Copy,
{
    pub fn new(data: Vec<T>) -> Self {
        RVector {
            data: data.clone(),
            valid: true,
        }
    }
}

#[cfg(test)]
#[path = "./tests/rvector_tests.rs"]
mod rvector_tests;
