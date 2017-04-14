
pub fn add(a: i32, b: i32) -> i32 {
    /// Add two signed integers.
    return a + b;
}


pub fn multiply(a: i32, number_times: i32) -> i32 {
    /// Implementation of multiplication using addition.
    let mut sum: i32 = 0;
    for _ in 0..number_times {
        println!("{}", sum);
        sum = add(sum, a);
    }
    return sum;
}


/// This marker tells the compiler to compile module below only when running tests.
#[cfg(test)]
mod test {

    /// We need to use "use" declaration, because any other module is unaware of our "functions" module by default.
    use functions;    
    /// Add
    #[test]
    fn test_add_positive_result() {
        assert_eq!(2, functions::add(1, 1));
    }

    #[test]
    fn test_add_negative_result() {
        assert_eq!(0, functions::add(-1, 1));
    }

    #[test]
    #[should_panic]
    fn test_add_panic() {
        /// This one should fail.
        assert_eq!(3, functions::add(1, 1));
    }

}