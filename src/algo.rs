use std::usize;

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

pub fn two_crystal_balls(breaks_array: &[bool]) -> usize {

    let jump_count = f32::sqrt(breaks_array.len() as f32) as usize;
    let mut i = jump_count;
    while i < breaks_array.len() {
        if breaks_array[i] {
            break;
        }
        i += jump_count;
    }

    let j_start = i - jump_count;

    for j in j_start..i  {
        if breaks_array[j] {
            return j;
        }
    }

    return usize::MAX;
}
