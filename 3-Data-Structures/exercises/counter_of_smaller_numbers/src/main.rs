fn main() {
    let nums = vec![5,2,6,1];
    let answer = vec![2,1,1,0];
    assert_eq!(answer, count_smaller(nums));

    let nums = vec![-1];
    let answer = vec![0];
    assert_eq!(answer, count_smaller(nums));

    let nums = vec![-1,-1];
    let answer = vec![0,0];
    assert_eq!(answer, count_smaller(nums));

    let nums = vec![26,78,27,100,33,67,90,23,66,5,38,7,35,23,52,22,83,51,98,69,81,32,78,28,94,13,2,97,3,76,99,51,9,21,84,66,65,36,100,41];
    let answer = vec![10,27,10,35,12,22,28,8,19,2,12,2,9,6,12,5,17,9,19,12,14,6,12,5,12,3,0,10,0,7,8,4,0,0,4,3,2,0,1,0];
    assert_eq!(answer, count_smaller(nums));
}

use counter_of_smaller_numbers::fenwich_tree;

fn count_smaller(nums: Vec<i32>) -> Vec<i32> {

}
// O(n^2)
// fn count_smaller(nums: Vec<i32>) -> Vec<i32> {
//     let nums_len = nums.iter().len();
//     let mut result = vec![0;nums_len];
//     if nums_len == 1 {
//         return result;
//     }
//
//     // keep track of all seen numbers on a sorted array
//     let mut seen_numbers = vec![nums[nums_len-1]]; // register the first number since
//                                                 // there is none to compare
//     
//     // process backwards
//     for i in (0..nums_len-1).rev() {
//         let index = seen_numbers.partition_point(|&x| x < nums[i]); // get the index where number
//                                                                     // should be inserted
//         seen_numbers.insert(index, nums[i]);
//         result[i] = index as i32;
//     }
//     result
// }
