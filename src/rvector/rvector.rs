use super::RVector;

impl<T> RVector<T>
where
    T: Copy,
{
    pub fn new(data: Vec<T>) -> Self {
        RVector {
            data: Ok(data.clone()),
        }
    }
    //pub fn magnitude(&self)
}
#[cfg(test)]
#[path = "./tests/rvector_tests.rs"]
mod rvector_tests;
