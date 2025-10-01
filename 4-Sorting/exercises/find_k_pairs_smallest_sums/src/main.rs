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

struct Solution;

struct MinHeap {
    elems: Vec<(i32, usize, usize)>,
}

impl MinHeap {
    pub fn new() -> Self {
        MinHeap { elems: Vec::new() }
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

    fn bubble_up(&mut self, i: usize) {
        if let Some(parent_index) = self.parent(i) && self.elems[parent_index].0 > self.elems[i].0 {
            self.elems.swap(i, parent_index);
            self.bubble_up(parent_index);
        }
    }

    pub fn push(&mut self, val: (i32, usize, usize)) {
        self.elems.push(val);
        let i = self.elems.len() - 1;
        self.bubble_up(i);

    }

    fn bubble_down(&mut self, i: usize) {
        let mut smallest = i;

        if let Some(child_index) = self.left_child(i) && self.elems[child_index].0 < self.elems[smallest].0 {
            smallest = child_index;
        }
        if let Some(child_index) = self.right_child(i) && self.elems[child_index].0 < self.elems[smallest].0 {
            smallest = child_index;
        }

        if smallest != i {
            self.elems.swap(smallest, i);
            self.bubble_down(smallest);
        }
    }

    pub fn pop(&mut self) -> Option<(i32, usize, usize)> {
        if self.elems.len() <= 1 {
            return self.elems.pop();
        }

        let min = self.elems.swap_remove(0);
        self.bubble_down(0);

        Some(min)
    }

    pub fn peek(&self) -> Option<&(i32, usize, usize)> {
        self.elems.iter().peekable().peek().map(|v| &**v)
    }
}

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, mut k: i32) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut min_heap = MinHeap::new();

        for (i, num1) in nums1.iter().enumerate().take(k as usize) {
            min_heap.push( (num1+nums2[0], i, 0) );
        }

        while k > 0 && min_heap.peek().is_some() {
            let smallest = min_heap.pop().unwrap();
            let pair = vec![nums1[smallest.1], nums2[smallest.2]];
            result.push(pair);

            k -= 1;

            if smallest.2 + 1 < nums2.len() { // if there is a next element in nums2
                min_heap.push( (nums1[smallest.1] + nums2[smallest.2+1], smallest.1, smallest.2+1) );
            }
        }

        result
    }
}
