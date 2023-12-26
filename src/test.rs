#[cfg(test)]
mod tests {
    use crate::algo;
    #[test]
    fn linear_test() {
        let arr = [1, 5, 3, 6, 8, 10];
        assert_eq!(algo::linear_search(&arr, 10), true);
        assert_eq!(algo::linear_search(&arr, 11), false);
    }

    #[test]
    fn binary_search_test() {
        let arr = [1, 5, 3, 6, 8, 10];
        assert_eq!(algo::binary_search(&arr, 10), true);
        assert_eq!(algo::binary_search(&arr, 11), false);
    }
}

