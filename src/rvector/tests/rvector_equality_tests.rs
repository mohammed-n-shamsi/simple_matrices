use crate::rvector::RVector;

#[test]
fn compare_math_vectors() {
    let simple_data = vec![1, 2, 4, 5];
    let instantiated_int_vec = RVector::new(simple_data);
    let made_vec = RVector::new(vec![1, 2, 4, 5]);
    assert_eq!(instantiated_int_vec, made_vec);
}
