#[cfg(test)]
mod tests {
    use vctr::matrix::*;

    #[test]
    fn add_correctness_test() {
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected = vec![vec![6.0, 8.0], vec![10.0, 12.0]];

        assert_eq!(expected, matrix_a.add(matrix_b).unwrap());
    }

    #[test]
    fn sub_correctness_test() {
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected = vec![vec![-4.0, -4.0], vec![-4.0, -4.0]];

        assert_eq!(expected, matrix_a.sub(matrix_b).unwrap());
    }

    #[test]
    fn mul_correctness_test() {
        let matrix_a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let matrix_b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let expected = vec![vec![19.0, 22.0], vec![43.0, 50.0]];

        assert_eq!(expected, matrix_a.mul(matrix_b).unwrap());
    }
}
