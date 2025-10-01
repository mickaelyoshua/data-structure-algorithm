pub fn binary_search_recursive<T: Ord>(arr: &[T], key: T, low: usize, high: usize) -> Option<&T> {
    if low > high {
        return None;
    }


    let middle = (high + low) / 2;

    if arr[middle] == key {
        return Some(&arr[middle]);
    }

    if arr[middle] > key {
        if middle == 0 {
            return None;
        }
        binary_search_recursive(arr, key, low, middle-1)
    } else {
        binary_search_recursive(arr, key, middle+1, high)
    }
}

pub fn binary_search<T: Ord>(arr: &[T], key: T) -> Option<&T> {
    if arr.is_empty() {
        return None;
    }

    binary_search_recursive(arr, key, 0, arr.len()-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_array() {
        let arr: [i32; 0] = [];
        assert_eq!(binary_search(&arr, 5), None);
    }

    #[test]
    fn test_single_element_found() {
        let arr = [42];
        assert_eq!(binary_search(&arr, 42), Some(&42));
    }

    #[test]
    fn test_single_element_not_found() {
        let arr = [42];
        assert_eq!(binary_search(&arr, 10), None);
    }

    #[test]
    fn test_element_at_beginning() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 1), Some(&1));
    }

    #[test]
    fn test_element_at_end() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 5), Some(&5));
    }

    #[test]
    fn test_element_in_middle() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 3), Some(&3));
    }

    #[test]
    fn test_element_not_found_smaller() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 0), None);
    }

    #[test]
    fn test_element_not_found_larger() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(binary_search(&arr, 10), None);
    }

    #[test]
    fn test_element_not_found_in_gap() {
        let arr = [1, 3, 5, 7, 9];
        assert_eq!(binary_search(&arr, 4), None);
    }

    #[test]
    fn test_large_array() {
        let arr: Vec<i32> = (0..1000).collect();
        assert_eq!(binary_search(&arr, 500), Some(&500));
        assert_eq!(binary_search(&arr, 0), Some(&0));
        assert_eq!(binary_search(&arr, 999), Some(&999));
        assert_eq!(binary_search(&arr, 1000), None);
    }

    #[test]
    fn test_with_strings() {
        let arr = ["apple", "banana", "cherry", "date", "elderberry"];
        assert_eq!(binary_search(&arr, "cherry"), Some(&"cherry"));
        assert_eq!(binary_search(&arr, "grape"), None);
    }

    #[test]
    fn test_two_elements_first() {
        let arr = [10, 20];
        assert_eq!(binary_search(&arr, 10), Some(&10));
    }

    #[test]
    fn test_two_elements_second() {
        let arr = [10, 20];
        assert_eq!(binary_search(&arr, 20), Some(&20));
    }

    #[test]
    fn test_two_elements_not_found() {
        let arr = [10, 20];
        assert_eq!(binary_search(&arr, 15), None);
    }
}