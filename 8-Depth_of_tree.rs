// 8. Given a binary tree, implement a function that returns the maximum depth of the tree.

#[derive(Debug)]
struct TreeNode {
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

impl TreeNode {
    fn new() -> Self {
        TreeNode { left: None, right: None }
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
    //      / \
    //     15  7
    let mut root = TreeNode::new();
    root.left = Some(Box::new(TreeNode::new()));
    root.right = Some(Box::new(TreeNode::new()));
    root.right.as_mut().unwrap().left = Some(Box::new(TreeNode::new()));
    root.right.as_mut().unwrap().right = Some(Box::new(TreeNode::new()));

    println!("Maximum depth of the tree: {}", max_depth(Some(Box::new(root))));
}
