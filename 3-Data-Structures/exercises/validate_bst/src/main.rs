type Link = Option<Box<TreeNode>>;

struct TreeNode {
    val: i32,
    left: Link,
    right: Link,
}

fn main() {
    let left_tree = Some(Box::new(TreeNode {
        val: 1,
        left: None,
        right: None,
    }));
    let right_tree = Some(Box::new(TreeNode {
        val: 3,
        left: None,
        right: None,
    }));

    let root = TreeNode {
        val: 2,
        left: left_tree,
        right: right_tree,
    };


}
