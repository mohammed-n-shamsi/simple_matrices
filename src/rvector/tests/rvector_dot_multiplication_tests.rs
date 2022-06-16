use crate::rvector::RVector;
#[test]
fn mul_vectors_both_consume() {
    let simple_data: RVector<i32> = RVector {
        data: vec![1, 2, 4, 5],
        valid: true,
    };
    let to_mul: RVector<i32> = RVector {
        data: vec![5, 4, 2, 1],
        valid: true,
    };
    let true_result: RVector<i32> = RVector {
        data: vec![26],
        valid: true,
    };

    let result = simple_data * to_mul;
    assert!(result.valid);
    assert_eq!(result.data, true_result.data);
}
#[test]
fn add_vectors_neither_consume() {
    let simple_data: RVector<i32> = RVector {
        data: vec![1, 2, 4, 5],
        valid: true,
    };
    let to_mul: RVector<i32> = RVector {
        data: vec![5, 4, 2, 1],
        valid: true,
    };

    let result = &simple_data * &to_mul;
    assert!(result.valid);
    assert_eq!(result.data, (simple_data * to_mul).data);
}
