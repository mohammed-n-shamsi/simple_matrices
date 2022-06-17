mod rmatrix;
mod rvector;

#[derive(Debug, Clone, PartialEq)]
pub enum OpErrors {
    InvalidInputs,
    MismatchedSizes,
}
