use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Option<Rc<RefCell<ListNode>>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

fn delete_node(node: Option<Rc<RefCell<ListNode>>>) {
    if let Some(node) = node {
        if let Some(next_node) = node.borrow_mut().next.take() {
            node.borrow_mut().val = next_node.borrow().val;
            node.borrow_mut().next = next_node.borrow_mut().next.take();
        }
    }
}

fn main() {
    // Creating a linked list: 1 -> 2 -> 3 -> 4
    let head = Rc::new(RefCell::new(ListNode::new(1)));
    let node_to_delete = Rc::new(RefCell::new(ListNode::new(2)));
    let next_node = Rc::new(RefCell::new(ListNode::new(3)));
    head.borrow_mut().next = Some(node_to_delete.clone());
    node_to_delete.borrow_mut().next = Some(next_node.clone());

    // Deleting node with value 2
    delete_node(Some(node_to_delete.clone()));

    // Printing the updated linked list: 1 -> 3 -> 4
    let mut current = Some(head);
    while let Some(node) = current {
        println!("{}", node.borrow().val);
        current = node.borrow().next.clone();
    }
}
