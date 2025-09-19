use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<TreeNode>>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn main() {
    let left_tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    })));
    let right_tree = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    })));

    let root = Some(Rc::new(RefCell::new(TreeNode {
        val: 2,
        left: left_tree,
        right: right_tree,
    })));

    println!("is valid: {}", is_valid_bts(root));
}

fn is_valid_bts(root: Link) -> bool {
    validate(&root, i64::MIN, i64::MAX)
}

fn validate(root: &Link, min: i64, max: i64) -> bool {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        let node_val = node.val as i64;
        if node_val <= min || node_val >= max {
            false
        } else {
            validate(&node.left, min, node_val) && validate(&node.right, node_val, max)
        }
    } else {
        true
    }
}
