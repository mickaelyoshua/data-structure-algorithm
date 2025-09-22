use construct_binary_tree::solution::{Solution, TreeNode};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];
    let tree = Solution::build_tree(preorder, inorder);

    let expected_tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    })));

    assert_eq!(tree, expected_tree);
    println!("Assertion passed: The refactored code is correct!");
}
