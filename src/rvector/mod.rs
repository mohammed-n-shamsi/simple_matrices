pub mod rvector;
pub mod rvector_addition;
pub mod rvector_dot_multiplication;
pub mod rvector_equality;
pub mod rvector_subtraction;

pub struct RVector<T> {
    data: Vec<T>,
    valid: bool,
}
