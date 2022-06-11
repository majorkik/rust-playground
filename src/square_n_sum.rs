#[cfg(test)]
mod square_sum_test {
    fn square_sum(vec: Vec<i32>) -> i32 {
        vec.iter().fold(0, |sum, &val| sum + val.pow(2))
    }

    #[test]
    fn square_sum_test() {
        assert_eq!(square_sum(vec![1, 2]), 5);
        assert_eq!(square_sum(vec![-1, -2]), 5);
        assert_eq!(square_sum(vec![5, 3, 4]), 50);
        assert_eq!(square_sum(vec![]), 0);
    }
}
