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
    pub fn distribute_coins(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (moves, _) = Self::dfs(&root);
        moves
    }

    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = node {
            let node = node.borrow();
            let (left_moves, left_balance) = Self::dfs(&node.left);
            let (right_moves, right_balance) = Self::dfs(&node.right);
            let balance = node.val + left_balance + right_balance - 1;
            (left_moves + right_moves + balance.abs(), balance)
        } else {
            (0, 0)
        }
    }
}


fn main() {
    println!("Hello, world!");


}
