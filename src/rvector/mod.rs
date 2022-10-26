pub mod rvector_addition;
pub mod rvector_equality;
pub mod rvector_std;
pub mod rvector_subtraction;

use crate::OpErrors;
#[derive(Debug)]
pub struct RVector<T> {
    data: Result<Vec<T>, OpErrors>,
}
