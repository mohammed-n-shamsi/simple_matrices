use crate::rvector::RVector;
#[test]
fn sub_vectors_both_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);

    let to_sub: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);

    let true_result: RVector<i32> = RVector::new(vec![-4, -2, 2, 4]);

    let result = simple_data - to_sub;
    assert_eq!(result, true_result);
}
#[test]
fn add_vectors_neither_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);

    let to_sub: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);

    let result = &simple_data - &to_sub;
    assert_eq!(result, (simple_data - to_sub));
}
