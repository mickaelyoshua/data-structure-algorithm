use std::collections::VecDeque;
use std::fmt::Debug;

pub fn selection_sort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let arr_len = arr.len();

    for i in 0..arr_len {
        let mut min_index = i;
        for j in (i + 1)..arr_len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}

fn merge<T: PartialOrd + Clone + Debug>(arr: &mut [T], low: usize, middle: usize, high: usize) {
    let mut queue1: VecDeque<T> = arr[low..=middle].iter().cloned().collect();
    let mut queue2: VecDeque<T> = arr[(middle + 1)..=high].iter().cloned().collect();

    let mut i = low;
    while !queue1.is_empty() && !queue2.is_empty() {
        if queue1.front().unwrap() <= queue2.front().unwrap() {
            arr[i] = queue1.pop_front().unwrap();
        } else {
            arr[i] = queue2.pop_front().unwrap();
        }
        i += 1;
    }

    while !queue1.is_empty() {
        arr[i] = queue1.pop_front().unwrap();
        i += 1;
    }
    while !queue2.is_empty() {
        arr[i] = queue2.pop_front().unwrap();
        i += 1;
    }
}

/// Sorts a slice using the merge sort algorithm.
/// This is a wrapper function that provides a convenient public API.
pub fn merge_sort<T: PartialOrd + Clone + Debug>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        merge_sort_recursive(arr, 0, len - 1);
    }
}

/// The recursive implementation of merge sort.
fn merge_sort_recursive<T: PartialOrd + Clone + Debug>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let middle = low + (high - low) / 2;
        merge_sort_recursive(arr, low, middle);
        merge_sort_recursive(arr, middle + 1, high);

        merge(arr, low, middle, high);
    }
}

fn partition<T: PartialOrd>(arr: &mut [T], low: usize, high: usize) -> usize {
    let mut first_high = low;
    for i in low..high {
        // Lomuto Partition Scheme
        if arr[i] < arr[high] {
            arr.swap(i, first_high);
            first_high += 1;
        }
    }

    arr.swap(high, first_high);
    first_high
}

fn quick_sort_recursive<T: PartialOrd + Debug>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        if p > 0 {
            quick_sort_recursive(arr, low, p-1);
        }
        quick_sort_recursive(arr, p+1, high);
    }
}

pub fn quick_sort<T: PartialOrd + Debug>(arr: &mut [T]) {
    let len = arr.len();
    if len > 1 {
        quick_sort_recursive(arr, 0, len-1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_sort_test(sort_fn: fn(&mut [i32])) {
        // Test case 1: Basic unsorted array
        let mut arr1 = vec![5, 3, 8, 4, 2, 9, 1, 7, 6];
        let sorted_arr1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        sort_fn(&mut arr1);
        assert_eq!(arr1, sorted_arr1, "Test Case 1 Failed: Basic Unsorted");

        // Test case 2: Array with duplicates
        let mut arr2 = vec![5, 3, 8, 4, 2, 9, 1, 7, 6, 3, 5];
        let sorted_arr2 = vec![1, 2, 3, 3, 4, 5, 5, 6, 7, 8, 9];
        sort_fn(&mut arr2);
        assert_eq!(arr2, sorted_arr2, "Test Case 2 Failed: Duplicates");

        // Test case 3: Already sorted array
        let mut arr3 = vec![1, 2, 3, 4, 5];
        let sorted_arr3 = vec![1, 2, 3, 4, 5];
        sort_fn(&mut arr3);
        assert_eq!(arr3, sorted_arr3, "Test Case 3 Failed: Already Sorted");

        // Test case 4: Reverse sorted array
        let mut arr4 = vec![5, 4, 3, 2, 1];
        let sorted_arr4 = vec![1, 2, 3, 4, 5];
        sort_fn(&mut arr4);
        assert_eq!(arr4, sorted_arr4, "Test Case 4 Failed: Reverse Sorted");

        // Test case 5: Empty array
        let mut arr5: Vec<i32> = vec![];
        let sorted_arr5: Vec<i32> = vec![];
        sort_fn(&mut arr5);
        assert_eq!(arr5, sorted_arr5, "Test Case 5 Failed: Empty");

        // Test case 6: Single element array
        let mut arr6 = vec![42];
        let sorted_arr6 = vec![42];
        sort_fn(&mut arr6);
        assert_eq!(arr6, sorted_arr6, "Test Case 6 Failed: Single Element");
    }

    #[test]
    fn test_selection_sort() {
        run_sort_test(selection_sort);
    }

    #[test]
    fn test_merge_sort() {
        run_sort_test(merge_sort);
    }

    #[test]
    fn test_quick_sort() {
        run_sort_test(quick_sort);
    }
}
