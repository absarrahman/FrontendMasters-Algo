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

pub fn binary_search<T>(arr: &[T], target_value: T) -> bool
where
    T: PartialEq + Ord + Clone,
{
    let low = 0;
    let high = arr.len();

    return recursive_binary_search(arr, target_value, &low, &high);
}

fn recursive_binary_search<T>(
    arr: &[T],
    target_value: T,
    low: &usize,
    high: &usize,
) -> bool
where
    T: PartialEq + Ord,
{
    if low >= high {
        return false;
    }

    let mid = *low + (*high - *low) / 2;
    let value = &arr[mid];

    if value == &target_value {
        return true;
    } else if value > &target_value {
        return recursive_binary_search(arr, target_value, low, &mid);
    } else {
        return recursive_binary_search(arr, target_value, &(mid + 1), high);
    }
}
