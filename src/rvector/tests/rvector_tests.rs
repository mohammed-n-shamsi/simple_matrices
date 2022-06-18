use crate::rvector::RVector;

#[test]
fn instantiate_math_vectors() {
    let simple_data = vec![1, 2, 4, 5];
    let instantiated_int_vec = RVector::new(simple_data);
    let made_vec = RVector::new(vec![1, 2, 4, 5]);
    assert_eq!(instantiated_int_vec, made_vec);
}

#[test]
fn test_mag_squared() {
    let simple_data = RVector::new(vec![1, 2, 4, 5]);
    let simple_mag_squared = RVector::magnitude_squared(&simple_data);
    assert_eq!(simple_mag_squared, Ok(46));
}

#[test]
fn test_element_addition() {
    let simple_data = RVector::new(vec![1, 2, 4, 5]);
    let addend_data = RVector::new(vec![5, 5, 5, 5]);
    let addend = 5;
    let simple_incremented = RVector::element_addition(&simple_data, addend);
    assert_eq!(simple_incremented, simple_data + addend_data);
}

#[test]
fn test_element_subtraction() {
    let simple_data = RVector::new(vec![1, 2, 4, 5]);
    let subtrahend_data = RVector::new(vec![5, 5, 5, 5]);
    let subtrahend = 5;
    let simple_decremented = RVector::element_subtraction(&simple_data, subtrahend);
    assert_eq!(simple_decremented, simple_data - subtrahend_data);
}

#[test]
fn test_element_division() {
    let simple_data = RVector::new(vec![2, 4, 6, 8]);
    let simple_exact_quotient = RVector::new(vec![1, 2, 3, 4]);
    let divisor = 2;
    let simple_quotient = RVector::element_division(&simple_data, divisor);
    assert_eq!(simple_quotient, simple_exact_quotient);
}

#[test]
fn test_element_multiplication() {
    let simple_data = RVector::new(vec![2, 4, 6, 8]);
    let simple_exact_product = RVector::new(vec![4, 8, 12, 16]);
    let multiplier = 2;
    let simple_product = RVector::element_multiplication(&simple_data, multiplier);
    assert_eq!(simple_product, simple_exact_product);
}
