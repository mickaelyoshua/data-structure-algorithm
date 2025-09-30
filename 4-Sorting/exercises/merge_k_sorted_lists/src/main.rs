// Merge K Sorted Lists
// https://leetcode.com/problems/merge-k-sorted-lists/description/

fn main() {
    let input = vec![
        create_list(ListNode::from_vec(vec![1,4,5])),
        create_list(ListNode::from_vec(vec![1,3,4])),
        create_list(ListNode::from_vec(vec![2,6])),
    ];
    let answer = create_list(ListNode::from_vec(vec![1,1,2,3,4,4,5,6]));
    let output = Solution::merge_k_lists(input);
    
    assert_eq!(answer, output);
}

#[derive(PartialEq, Eq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode {
            val,
            next: None,
        }
    }
    
    pub fn from_vec(vals: Vec<i32>) -> Self {
        unimplemented!()
    }
}

pub fn create_list(node: ListNode) -> Option<Box<ListNode>> {
    Some(Box::new(node))
}

struct Solution;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        unimplemented!()
    }
}
