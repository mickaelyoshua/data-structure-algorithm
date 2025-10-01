// Find K Pairs with Smallest Sums
// https://leetcode.com/problems/find-k-pairs-with-smallest-sums/description/

fn main() {
    let nums1 = vec![1,7,11];
    let nums2 = vec![2,4,6];
    let k = 3;
    let answer = vec![vec![1,2],vec![1,4],vec![1,6]];
    let output = Solution::k_smallest_pairs(nums1, nums2, k);

    assert_eq!(answer, output);
}

struct MinHeap {
    elems: Vec<i32>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { elems: Vec::new() }
    }

    fn bubble_up(&mut self, i: usize) {
        if let Some(parent_index) = self.parent(i) && self.elems[parent_index] > self.elems[i] {
            self.elems.swap(i, parent_index);
            self.bubble_up(parent_index);
        }
    }

    pub fn push(&mut self, val: i32) {
        self.elems.push(val);
        let i = self.elems.len() - 1;
        self.bubble_up(i);

    }

    fn bubble_down(&mut self, i: usize) {

    }

    pub fn pop(&mut self) -> Option<i32> {
        if self.elems.len() <= 1 {
            return self.elems.pop();
        }

        let value = self.elems[0];
        self.elems[0] = self.elems.pop().unwrap();

        self.bubble_down(0);

        Some(value)
    }



    fn parent(&self, i: usize) -> Option<usize> {
        if i == 0 {
            return None;
        }
        Some( (i - 1) / 2 )
    }

    fn left_child(&self, i: usize) -> Option<usize> {
        let child = i * 2 + 1;
        if child > self.elems.len() - 1 {
            return None;
        }
        Some(child)
    }

    fn right_child(&self, i: usize) -> Option<usize> {
        let child = i * 2 + 2;
        if child > self.elems.len() - 1 {
            return None;
        }
        Some(child)
    }
}


struct Solution;
impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        

        unimplemented!()
    }
}
