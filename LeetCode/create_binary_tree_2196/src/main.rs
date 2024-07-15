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
use std::collections::HashMap;
struct Solution(Vec<Vec<i32>>);

impl Solution {
    pub fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = HashMap::new();
        let mut children = HashMap::new();

        for description in descriptions {
            let (parent_val, child_val, is_left) = (description[0], description[1], description[2]);

            // Ensure parent node exists
            if !nodes.contains_key(&parent_val) {
                nodes.insert(parent_val, Rc::new(RefCell::new(TreeNode::new(parent_val))));
            }
            // Ensure child node exists
            if !nodes.contains_key(&child_val) {
                nodes.insert(child_val, Rc::new(RefCell::new(TreeNode::new(child_val))));
            }

            let parent_node = nodes.get(&parent_val).unwrap();
            let child_node = nodes.get(&child_val).unwrap();

            if is_left == 1 {
                parent_node.borrow_mut().left = Some(Rc::clone(child_node));
            } else {
                parent_node.borrow_mut().right = Some(Rc::clone(child_node));
            }

            children.insert(child_val, ());
        }

        // Find root (a node that is never a child)
        let root = nodes.into_iter().find_map(|(val, node)| {
            if !children.contains_key(&val) {
                Some(node)
            } else {
                None
            }
        });

        root
    }
}

fn main() {
    println!("Hello, world!");

    let descriptions = vec![
        vec![20, 15, 1],
        vec![20, 17, 0],
        vec![50, 20, 1],
        vec![50, 80, 0],
        vec![80, 19, 1],
    ];

    let root = Solution::create_binary_tree(descriptions);
    println!("{:?}", root);
}
