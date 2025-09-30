// Queue Reconstruction by Height
// https://leetcode.com/problems/queue-reconstruction-by-height/description/

fn main() {
    let input = vec![vec![7,0],vec![4,4],vec![7,1],vec![5,0],vec![6,1],vec![5,2]];
    let answer = vec![vec![5,0],vec![7,0],vec![5,2],vec![6,1],vec![4,4],vec![7,1]];
    let output = Solution::reconstruct_queue(input);
    assert_eq!(output, answer);
}

struct Solution;

pub fn quick_sort(arr: &mut [Vec<i32>]) {
    let len = arr.len();
    if len > 1 {
        quick_sort_recursive(arr, 0, len-1)
    }
}

pub fn quick_sort_recursive(arr: &mut [Vec<i32>], low: usize, high: usize) {
    if low < high {
        let p = partition(arr, low, high);
        if p > 0 {
            quick_sort_recursive(arr, low, p-1);
        }
        quick_sort_recursive(arr, p+1, high);
    }
}

pub fn partition(arr: &mut [Vec<i32>], low: usize, high: usize) -> usize {
    let mut first_high = low;
    for i in low..high {
        // Lomuto Partition - the pivot is the last element
        if (arr[i][0] > arr[high][0]) || (arr[i][0] == arr[high][0] && arr[i][1] < arr[high][1]) {
            arr.swap(i, first_high);
            first_high += 1;
        }
    }
    arr.swap(first_high, high);
    first_high
}

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        // people.sort_by(|a,b|{
        //     match b[0].cmp(&a[0]) {
        //         std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
        //         other => other,
        //     }
        // });

        quick_sort(&mut people);

        let mut result: Vec<Vec<i32>> = Vec::with_capacity(people.len());

        for person in people.into_iter() {
            let index = person[1] as usize;
            result.insert(index, person);
        }
        
        result
    }
}
