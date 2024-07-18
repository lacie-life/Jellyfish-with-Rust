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
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        fn dfs(node: &Option<Rc<RefCell<TreeNode>>>, distance: i32, count: &mut i32) -> Vec<i32> {
            if let Some(node) = node {
                let node = node.borrow();
                if node.left.is_none() && node.right.is_none() {
                    let mut dists = vec![0; distance as usize];
                    dists[0] = 1;
                    return dists;
                }
                let left_dists = dfs(&node.left, distance, count);
                let right_dists = dfs(&node.right, distance, count);

                for i in 0..distance as usize {
                    for j in 0..distance as usize - i {
                        if i + j + 2 <= distance as usize {
                            *count += left_dists[i] * right_dists[j];
                        }
                    }
                }

                let mut dists = vec![0; distance as usize];
                for i in 0..distance as usize - 1 {
                    dists[i + 1] += left_dists[i] + right_dists[i];
                }
                return dists;
            }
            vec![0; distance as usize]
        }

        let mut count = 0;
        dfs(&root, distance, &mut count);
        count
    }
}


fn main() {
    println!("Hello, world!");

    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));

    let distance = 3;
    let result = Solution::count_pairs(Some(root), distance);
    println!("result = {}", result);


    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let distance = 3;

    let result = Solution::count_pairs(Some(root), distance);
    println!("result = {}", result);
}
