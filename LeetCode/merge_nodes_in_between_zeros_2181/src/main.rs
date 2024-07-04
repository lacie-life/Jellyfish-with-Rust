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

struct Solution(Option<Box<ListNode>>);

impl Solution {
    pub fn merge_nodes(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy;
        let mut sum = 0;
        while let Some(node) = head {
            if node.val == 0 && sum != 0 {
                cur.next = Some(Box::new(ListNode::new(sum)));
                cur = cur.as_mut().next.as_mut().unwrap();
                sum = 0;
            }
            sum += node.val;
            head = node.next;
        }
        dummy.next.take()
    }
}

fn create_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = Some(Box::new(ListNode::new(0)));
    let mut current = &mut head;
    for &val in vec.iter() {
        if let Some(node) = current {
            node.next = Some(Box::new(ListNode::new(val)));
            current = &mut node.next;
        }
    }
    head.unwrap().next
}

fn main() {
    println!("Hello, world!");

    let input_list = create_list(vec![0, 3, 1, 0, 4, 5, 2, 0]);

    let solution = Solution::merge_nodes(input_list);
    println!("{:?}", solution);

}
