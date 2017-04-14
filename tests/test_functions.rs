extern crate my_unittests;


#[cfg(test)]
mod integration_tests {

    use my_unittests::functions;

    #[test]
    fn test_multiply1() {
        assert_eq!(4, functions::multiply(2, 2));
    }

    #[test]
    fn test_multiply2() {
        assert_eq!(100, functions::multiply(10, 10));
    }

}