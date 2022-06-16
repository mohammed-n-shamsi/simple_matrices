use crate::rvector::RVector;

#[test]
fn instantiate_math_vectors() {
    let simple_data = vec![1, 2, 4, 5];
    let instantiated_int_vec = RVector::new(simple_data);
    let made_vec = RVector {
        data: vec![1, 2, 4, 5],
        valid: true,
    };
    assert_eq!(instantiated_int_vec.data, made_vec.data);
}
