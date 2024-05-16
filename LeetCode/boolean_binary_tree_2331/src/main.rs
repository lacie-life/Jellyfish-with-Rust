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
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
struct Solution (Option<Rc<RefCell<TreeNode>>>);

impl Solution {
    pub fn evaluate_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let root = root.unwrap();
        let root = root.borrow();
        if root.left.is_none() && root.right.is_none() {
            return root.val == 1;
        }
        let left = Self::evaluate_tree(root.left.clone());
        let right = Self::evaluate_tree(root.right.clone());
        if root.val == 2 {
            left || right
        } else {
            left && right
        }
    }
}


fn main() {
    println!("Hello, world!");

    let mut root = TreeNode::new(1);
}
