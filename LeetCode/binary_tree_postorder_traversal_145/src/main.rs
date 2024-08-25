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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn helper(node: &Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(n) = node {
                let n = n.borrow();
                helper(&n.left, result);
                helper(&n.right, result);
                result.push(n.val);
            }
        }

        let mut result = Vec::new();
        helper(&root, &mut result);
        result
    }
}


fn main() {
    println!("Hello, world!");

    let mut root = TreeNode::new(1);
    let mut right = TreeNode::new(2);
    let mut right_left = TreeNode::new(3);
    right_left.left = None;
    right_left.right = None;
    right.left = Some(Rc::new(RefCell::new(right_left)));
    right.right = None;
    root.left = None;
    root.right = Some(Rc::new(RefCell::new(right)));

    let result = Solution::postorder_traversal(Some(Rc::new(RefCell::new(root))));
    println!("{:?}", result);
}
