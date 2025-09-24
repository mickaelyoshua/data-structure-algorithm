pub fn selection_sort<T: PartialOrd>(arr: &mut [T]) {
    let arr_len = arr.len();

    for i in 0..arr_len {
        let mut min_index = i;
        for j in (i+1)..arr_len {
            if arr[j] < arr[min_index] {
                min_index = j;
            }
        }
        if i != min_index {
            arr.swap(i, min_index);
        }
    }
}
