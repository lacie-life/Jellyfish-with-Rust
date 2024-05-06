// Define the ListNode struct
#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Define the remove_nodes function
pub fn remove_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Reverse the linked list
    let mut prev = None;
    let mut curr = head;
    while let Some(mut boxed_node) = curr.take() {
        let next = boxed_node.next.take();
        boxed_node.next = prev.take();
        prev = Some(boxed_node);
        curr = next;
    }
    head = prev;

    // Traverse the reversed list and remove nodes
    let mut max_val = head.as_ref()?.val;
    let mut node = head.as_mut()?;
    while let Some(next) = node.next.as_mut() {
        if next.val < max_val {
            node.next = next.next.take();
        } else {
            max_val = next.val;
            node = node.next.as_mut().unwrap();
        }
    }

    // Reverse the list again to restore the original order
    prev = None;
    curr = head;
    while let Some(mut boxed_node) = curr.take() {
        let next = boxed_node.next.take();
        boxed_node.next = prev.take();
        prev = Some(boxed_node);
        curr = next;
    }

    prev
}

fn main() {
    // Example usage:
    let mut head = Some(Box::new(ListNode::new(5)));
    head.as_mut().unwrap().next = Some(Box::new(ListNode::new(2)));
    head.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(13)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(3)));
    head.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next.as_mut().unwrap().next = Some(Box::new(ListNode::new(8)));

    println!("Original list: {:?}", head);

    let modified_head = remove_nodes(head);
    println!("Modified list: {:?}", modified_head);
}

