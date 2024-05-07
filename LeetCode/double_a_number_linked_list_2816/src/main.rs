struct Solution (Option<Box<ListNode>>);
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
impl Solution {

    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut prev = None;
        let mut current = head.take();
        while current.is_some() {
            let mut next = current.as_mut().unwrap().next.take();
            current.as_mut().unwrap().next = prev.take();
            prev = current;
            current = next;
        }
        prev
    }

    pub fn double_it(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        // Reverse the list
        let head = Solution::reverse_list(head);

        // Traverse the list from the least significant digit
        let mut head = head;
        let mut carry = 0;
        let mut current = &mut head;
        while current.is_some() {
            let val = current.as_mut().unwrap().val;
            let sum = val * 2 + carry;
            current.as_mut().unwrap().val = sum % 10;
            carry = sum / 10;
            current = &mut current.as_mut().unwrap().next;
        }
        if carry > 0 {
            let new_node = Some(Box::new(ListNode::new(carry)));
            let mut current = &mut head;
            while current.is_some() {
                current = &mut current.as_mut().unwrap().next;
            }
            current.replace(new_node.unwrap());
        }

        // Reverse the list
        let head = Solution::reverse_list(head);
        head
    }
}

fn main() {
    println!("Hello, world!");

    // Input [1, 8, 9]
    let mut head = Some(Box::new(ListNode::new(9)));
    let mut current = &mut head;
    current.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));
    current = &mut current.as_mut().unwrap().next;
    current.as_mut().unwrap().next = Some(Box::new(ListNode::new(9)));

    head = Solution::double_it(head);
    let mut current = &head;
    while current.is_some() {
        println!("{}", current.as_ref().unwrap().val);
        current = &current.as_ref().unwrap().next;
    }


}
