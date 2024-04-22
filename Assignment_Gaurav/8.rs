//Given a binary tree, implement a function that returns the maximum depth of the tree.



// Definition for a binary tree node.
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

fn max_depth(root: Option<Box<TreeNode>>) -> i32 {
    match root {
        Some(node) => {
            let left_depth = max_depth(node.left);
            let right_depth = max_depth(node.right);
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    // Example tree:
    //     3
    //    / \
    //   9  20
    //     /  \
    //    15   7
    let mut root = TreeNode::new(3);
    let mut node1 = TreeNode::new(9);
    let mut node2 = TreeNode::new(20);
    let mut node3 = TreeNode::new(15);
    let node4 = TreeNode::new(7);
    node2.left = Some(Box::new(node3));
    node2.right = Some(Box::new(node4));
    root.left = Some(Box::new(node1));
    root.right = Some(Box::new(node2));

    println!("Max depth of the tree: {}", max_depth(Some(Box::new(root))));
}
