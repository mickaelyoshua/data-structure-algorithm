// https://leetcode.com/problems/sort-list/
// Given the head of a linked list, return the list after sorting it in ascending order.

fn main() {
    let input = ListNode::from_vec(vec![4,2,1,3]);
    let answer = ListNode::from_vec(vec![1,2,3,4]);

    let ordered_list = Solution::sort_list(Some(Box::new(input)));
    assert_eq!(*ordered_list.unwrap(), answer);
}

#[derive(PartialEq, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
       ListNode { val, next: None }
    }

    pub fn from_vec(vals: Vec<i32>) -> Self {
        let mut vals_iter = vals.into_iter();
        let mut list = Self::new(vals_iter.next().unwrap());
        for v in vals_iter {
            list.push(v);
        }

        list
    }

    pub fn push(&mut self, val: i32) {
        let mut last_node = &mut self.next;
        while let Some(node) = last_node {
            last_node = &mut node.next;
        }

        let node = ListNode {
            val,
            next: None
        };
        *last_node = Some(Box::new(node));
    }
}

struct Solution;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        head.as_ref()?;

        if head.as_ref().unwrap().next.is_none() {
            return head;
        }

        // Get list length
        let mut len = 0;
        let mut current = head.as_ref();
        while let Some(node) = current {
            len += 1;
            current = node.next.as_ref();
        }

        // Get mid node
        let mid = len / 2;
        let mut mid_node = head.as_mut().unwrap();
        for _ in 0..(mid-1) {
            mid_node = mid_node.next.as_mut().unwrap();
        }

        // Split
        let second_half = mid_node.next.take();
        // This will remove the node from the mid position and leave a None
        // resulting in two lists, one from head to None (previous mid) and another
        // from mid to the end of the list.
        
        let sorted_first = Self::sort_list(head);
        let sorted_second = Self::sort_list(second_half);

        Self::merge(sorted_first, sorted_second)
    }

    fn merge(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut list = ListNode::new(0);
        let mut tail = &mut list;
        
        while list1.is_some() && list2.is_some() {
            // Identify wich one is the smallest
            let smaller = if list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val {
                &mut list1
            } else {
                &mut list2
            };
            
            // Take the smaller node and replace with its next, then put the smaller node on the
            // new list to be returned
            let mut smaller_node = smaller.take().unwrap();
            *smaller = smaller_node.next.take(); // put the rest back to the list to be merged
            tail.next = Some(smaller_node);

            // Update tail
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };
        list.next // the first node is the dummy 0 value
    }
}
