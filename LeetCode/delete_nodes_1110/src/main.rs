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
use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let delete_set: HashSet<i32> = to_delete.into_iter().collect();
        let mut res = Vec::new();
        Self::pre_order(root.clone(), &delete_set, true, &mut res);
        res
    }

    fn pre_order(
        node: Option<Rc<RefCell<TreeNode>>>,
        delete_set: &HashSet<i32>,
        is_new_root: bool,
        res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(n) = node.clone() {
            let mut n_borrowed = n.borrow_mut();
            let root_delete = delete_set.contains(&n_borrowed.val);
            if !root_delete && is_new_root {
                res.push(node.clone());
            }
            n_borrowed.left = Self::pre_order(n_borrowed.left.clone(), delete_set, root_delete, res);
            n_borrowed.right = Self::pre_order(n_borrowed.right.clone(), delete_set, root_delete, res);
            if root_delete {
                None
            } else {
                node.clone()
            }
        } else {
            None
        }
    }
}

// Utility function to print the tree level by level
fn print_trees(forests: &Vec<Option<Rc<RefCell<TreeNode>>>>) {
    for root in forests {
        if let Some(node) = root {
            let mut queue = vec![Some(node.clone())];
            while !queue.is_empty() {
                let current = queue.remove(0);
                if let Some(current_node) = current {
                    print!("{} ", current_node.borrow().val);
                    queue.push(current_node.borrow().left.clone());
                    queue.push(current_node.borrow().right.clone());
                }
            }
            println!();
        }
    }
}

fn main() {
    println!("Hello, world!");
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root.borrow_mut().left.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.borrow_mut().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.borrow_mut().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let to_delete = vec![3, 5];

    let result = Solution::del_nodes(Some(root), to_delete);

    println!("Remaining forests after deletion:");
    print_trees(&result);
}
