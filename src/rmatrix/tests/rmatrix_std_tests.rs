use crate::rmatrix::RMatrix;
use crate::rmatrix::RMatrixDetails;

#[test]
fn instantiate_math_matrix() {
    let simple_input = vec![1, 2, 4, 5];
    let simple_data = vec![
        simple_input.clone(),
        simple_input.clone(),
        simple_input.clone(),
        simple_input.clone(),
    ];
    let test_output = RMatrix {
        matrix: Ok(RMatrixDetails {
            data: simple_data.clone(),
            cols: 4,
            rows: 4,
        }),
    };

    assert_eq!(RMatrix::new(simple_data), test_output);
}
