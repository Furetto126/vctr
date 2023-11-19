#[cfg(test)]
mod tests {
    use vctr::vector::*;

    #[test]
    fn add_correctness_test() {
        let vector_a = vec![0.0, 1.0, 2.0];
        let vector_b = vec![3.0, 4.0, 5.0];
        let expected = vec![3.0, 5.0, 7.0];

        assert_eq!(expected, vector_a.add(vector_b).unwrap());
    }

    #[test]
    fn sub_correctness_test() {
        let vector_a = vec![0.0, 1.0, 2.0];
        let vector_b = vec![3.0, 4.0, 5.0];
        let expected = vec![-3.0, -3.0, -3.0];

        assert_eq!(expected, vector_a.sub(vector_b).unwrap());
    }

    #[test]
    fn mul_correctness_test() {
        let vector_a = vec![0.0, 1.0, 2.0];
        let vector_b = vec![3.0, 4.0, 5.0];
        let expected = vec![0.0, 4.0, 10.0];

        assert_eq!(expected, vector_a.mul(vector_b).unwrap());
    }

    #[test]
    fn div_correctness_test() {
        let vector_a = vec![0.0, 1.0, 2.0];
        let vector_b = vec![3.0, 4.0, 5.0];
        let expected = vec![0.0, 0.25, 2.0 / 5.0];

        assert_eq!(expected, vector_a.div(vector_b).unwrap());
    }
}
