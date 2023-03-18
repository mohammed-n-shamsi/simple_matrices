use crate::rmatrix::RMatrix;
use crate::rmatrix::RMatrixDetails;

#[test]
fn add_matrices_both_consume() {
    let simple_input = vec![1, 2, 3, 4, 5];
    let reverse_input = vec![5, 4, 3, 2, 1];
    let simple_data = vec![
        simple_input.clone(),
        reverse_input.clone(),
        simple_input.clone(),
        reverse_input.clone(),
        simple_input.clone(),
    ];

    let reverse_simple_data = vec![
        reverse_input.clone(),
        simple_input.clone(),
        reverse_input.clone(),
        simple_input.clone(),
        reverse_input.clone(),
    ];
    let my_mat = RMatrix::new(simple_data);
    let my_rev_mat = RMatrix::new(reverse_simple_data);

    let test_output_vec = vec![6, 6, 6, 6, 6];
    let test_output_mat = vec![
        test_output_vec.clone(),
        test_output_vec.clone(),
        test_output_vec.clone(),
        test_output_vec.clone(),
        test_output_vec.clone(),
    ];
    let test_output = RMatrix {
        matrix: Ok(RMatrixDetails {
            data: test_output_mat.clone(),
            cols: 5,
            rows: 5,
        }),
    };

    assert_eq!(my_mat + my_rev_mat, test_output);
}
