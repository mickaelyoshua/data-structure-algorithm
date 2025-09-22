use std::rc::Rc;
use std::cell::RefCell;

// The LeetCode environment provides the definitions for `TreeNode` and `Link`.
// You should not include them in the code you paste on the platform.

// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//     pub val: i32,
//     pub left: Link,
//     pub right: Link,
// }
// pub type Link = Option<Rc<RefCell<TreeNode>>>;

pub struct Solution;

// Definition for a binary tree node provided by LeetCode.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl Solution {
   pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
       if preorder.is_empty() {
           return None;
       }

       let root_val = preorder[0];
       let root_val_inorder_pos = inorder.iter().position(|&x| x == root_val).unwrap();

       let inorder_left_tree = &inorder[..root_val_inorder_pos];
       let inorder_right_tree = &inorder[(root_val_inorder_pos + 1)..];

       let preorder_left_tree = &preorder[1..(root_val_inorder_pos + 1)];
       let preorder_right_tree = &preorder[(root_val_inorder_pos + 1)..];

       let root_node = TreeNode {
           val: root_val,
           left: Self::build_tree(preorder_left_tree.to_vec(), inorder_left_tree.to_vec()),
           right: Self::build_tree(preorder_right_tree.to_vec(), inorder_right_tree.to_vec()),
       };
       
        Some(Rc::new(RefCell::new(root_node)))
    }
}
