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

struct Solution;

impl Solution {
    pub fn split_list_to_parts(head: Option<Box<ListNode>>, k: i32) -> Vec<Option<Box<ListNode>>> {
        let mut length = 0;
        let mut current = &head;

        // Calculate the total length of the linked list
        while let Some(node) = current {
            length += 1;
            current = &node.next;
        }

        let mut part_size = length / k;
        let mut extra_parts = length % k;

        let mut result = Vec::with_capacity(k as usize);
        let mut current = head;

        for _ in 0..k {
            let mut part_head = current.take();
            let mut part_tail = &mut part_head;

            for _ in 0..(part_size + if extra_parts > 0 { 1 } else { 0 }) - 1 {
                if let Some(node) = part_tail {
                    part_tail = &mut node.next;
                }
            }

            if let Some(node) = part_tail {
                current = node.next.take();
            }

            result.push(part_head);
            if extra_parts > 0 {
                extra_parts -= 1;
            }
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
}
