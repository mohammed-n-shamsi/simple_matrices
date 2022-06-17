use crate::rvector::RVector;
#[test]
fn add_vectors_both_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);
    let to_add: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);
    let true_result: RVector<i32> = RVector::new(vec![6, 6, 6, 6]);

    let result = simple_data + to_add;
    assert_eq!(result, true_result);
}
#[test]
fn add_vectors_neither_consume() {
    let simple_data: RVector<i32> = RVector::new(vec![1, 2, 4, 5]);
    let to_add: RVector<i32> = RVector::new(vec![5, 4, 2, 1]);

    let result = &simple_data + &to_add;
    assert_eq!(result, (simple_data + to_add));
}
