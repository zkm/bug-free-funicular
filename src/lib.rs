pub mod math {
    /// Sum a slice of i64 numbers.
    pub fn sum(nums: &[i64]) -> i64 {
        nums.iter().copied().sum()
    }

    #[cfg(test)]
    mod tests {
        use super::sum;

        #[test]
        fn sum_empty_is_zero() {
            assert_eq!(sum(&[]), 0);
        }

        #[test]
        fn sum_positive_and_negative() {
            assert_eq!(sum(&[5, -2, 7]), 10);
        }
    }
}
