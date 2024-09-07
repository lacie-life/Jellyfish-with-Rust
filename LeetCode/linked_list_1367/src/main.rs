// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
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

struct Solution;

impl Solution {
    fn dfs(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (head, root) {
            (Some(h), Some(r)) => {
                let r = r.borrow();
                if h.val == r.val {
                    Self::dfs(&h.next, &r.left) || Self::dfs(&h.next, &r.right)
                } else {
                    false
                }
            }
            (None, _) => true,
            (_, None) => false,
        }
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(r) => {
                Self::dfs(&head, &Some(r.clone())) ||
                    Self::is_sub_path(head.clone(), r.borrow().left.clone()) ||
                    Self::is_sub_path(head, r.borrow().right.clone())
            }
            None => false,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
