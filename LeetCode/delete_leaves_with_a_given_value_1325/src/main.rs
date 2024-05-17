use std::cell::RefCell;
use std::rc::Rc;

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node_rc) => {
                let mut node_ref = node_rc.borrow_mut();
                node_ref.left = Self::remove_leaf_nodes(node_ref.left.take(), target);
                node_ref.right = Self::remove_leaf_nodes(node_ref.right.take(), target);
                if node_ref.val == target && node_ref.left.is_none() && node_ref.right.is_none() {
                    None
                } else {
                    drop(node_ref);
                    Some(node_rc)
                }
            },
            None => None,
        }
    }
}

fn main() {
    // Example usage:
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right_right = Rc::new(RefCell::new(TreeNode::new(4)));

    root.borrow_mut().left = Some(left.clone());
    root.borrow_mut().right = Some(right.clone());
    left.borrow_mut().left = Some(left_left.clone());
    right.borrow_mut().left = Some(right_left.clone());
    right.borrow_mut().right = Some(right_right.clone());

    let result = Solution::remove_leaf_nodes(Some(root), 2);
    println!("{:?}", result);
}
