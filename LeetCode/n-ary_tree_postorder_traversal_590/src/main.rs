use std::cell::RefCell;
use std::rc::Rc;

// Definition for a Node.
#[derive(Debug)]
struct Node {
    val: i32,
    children: Vec<Option<Rc<RefCell<Node>>>>,
}

impl Node {
    fn new(val: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            val,
            children: vec![],
        }))
    }
}

struct Solution;

impl Solution {
    pub fn postorder(root: Option<Rc<RefCell<Node>>>) -> Vec<i32> {
        if root.is_none() {
            return vec![];
        }

        let mut ans = vec![];
        let mut stack = vec![root.unwrap()];

        while let Some(node) = stack.pop() {
            let node_ref = node.borrow();
            ans.push(node_ref.val);
            for child in node_ref.children.iter() {
                if let Some(child_node) = child {
                    stack.push(Rc::clone(child_node));
                }
            }
        }

        ans.reverse();
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let node5 = Node::new(5);
    let node6 = Node::new(6);
    let node3 = Node::new(3);
    node3.borrow_mut().children.push(Some(node5));
    node3.borrow_mut().children.push(Some(node6));
    let node2 = Node::new(2);
    let node4 = Node::new(4);
    let root = Node::new(1);
    root.borrow_mut().children.push(Some(node3));
    root.borrow_mut().children.push(Some(node2));
    root.borrow_mut().children.push(Some(node4));

    let result = Solution::postorder(Some(root));
    println!("{:?}", result); // Expected output: [5, 6, 3, 2, 4, 1]
}

