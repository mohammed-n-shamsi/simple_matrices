use crate::rvector::RVector;
#[test]
fn mul_vectors_both_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);
    let to_mul: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);

    let true_result = Ok(26);

    let result = simple_data * to_mul;
    assert_eq!(result, true_result);
}
#[test]
fn mul_vectors_neither_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);
    let to_mul: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);

    let result = &simple_data * &to_mul;
    assert_eq!(result, simple_data * to_mul);
}
