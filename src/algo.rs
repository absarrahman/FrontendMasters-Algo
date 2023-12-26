pub fn linear_search<T>(arr: &[T], target_value: T) -> bool
where
    T: PartialEq,
{
    for value in arr {
        if *value == target_value {
            return true;
        }
    }
    return false;
}
