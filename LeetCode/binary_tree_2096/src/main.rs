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

struct Solution(Option<Rc<RefCell<TreeNode>>>);

impl Solution {
    pub fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
        // Helper function to find LCA and paths
        fn find_paths(node: &Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32, path: &mut Vec<char>, paths: &mut Vec<Vec<char>>) -> bool {
            if node.is_none() { return false; }
            let node = node.as_ref().unwrap().borrow();

            if node.val == start_value || node.val == dest_value {
                paths.push(path.clone());
                if paths.len() == 2 { return true; }
            }

            path.push('L');
            if find_paths(&node.left, start_value, dest_value, path, paths) { return true; }
            path.pop();

            path.push('R');
            if find_paths(&node.right, start_value, dest_value, path, paths) { return true; }
            path.pop();

            if !paths.is_empty() && paths[0].len() <= path.len() {
                paths.push(path.clone());
                return true;
            }

            false
        }

        let mut paths = Vec::new();
        find_paths(&root, start_value, dest_value, &mut Vec::new(), &mut paths);

        if paths.len() != 2 { return String::new(); }

        let (mut path_to_start, mut path_to_dest) = (paths[0].clone(), paths[1].clone());
        while !path_to_start.is_empty() && !path_to_dest.is_empty() && path_to_start[0] == path_to_dest[0] {
            path_to_start.remove(0);
            path_to_dest.remove(0);
        }

        path_to_start.iter().map(|_| 'U').chain(path_to_dest.into_iter()).collect()
    }
}

fn build_tree(values: &[Option<i32>]) -> Option<Rc<RefCell<TreeNode>>> {
    use std::collections::VecDeque;

    if values.is_empty() || values[0].is_none() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(values[0].unwrap())));
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());

    let mut i = 1;
    while i < values.len() {
        let node = queue.pop_front().unwrap();
        if let Some(val) = values[i] {
            let left_child = Rc::new(RefCell::new(TreeNode::new(val)));
            node.borrow_mut().left = Some(left_child.clone());
            queue.push_back(left_child);
        }
        i += 1;
        if i < values.len() {
            if let Some(val) = values[i] {
                let right_child = Rc::new(RefCell::new(TreeNode::new(val)));
                node.borrow_mut().right = Some(right_child.clone());
                queue.push_back(right_child);
            }
            i += 1;
        }
    }
    Some(root)
}

fn main() {
    println!("Hello, world!");

    let root = build_tree(&[
        Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)
    ]);
    let start_value = 3;
    let dest_value = 6;

    let result = Solution::get_directions(root, start_value, dest_value);
    println!("{}", result);

}
