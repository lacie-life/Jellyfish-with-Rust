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

struct Solution (Option<Box<ListNode>>);

impl Solution {
    pub fn nodes_between_critical_points(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut min_distance = i32::MAX;
        let mut first_ma_index = -1;
        let mut prev_ma_index = -1;
        let mut index = 1;

        let mut prev = &head; // Point to the index 0.
        let mut curr = &head.as_ref().unwrap().next; // Point to the index 1.

        while let Some(curr_node) = curr {
            if let Some(next_node) = &curr_node.next {
                if (curr_node.val > prev.as_ref().unwrap().val && curr_node.val > next_node.val) ||
                    (curr_node.val < prev.as_ref().unwrap().val && curr_node.val < next_node.val) {
                    if first_ma_index == -1 {
                        first_ma_index = index;
                    }
                    if prev_ma_index != -1 {
                        min_distance = min_distance.min(index - prev_ma_index);
                    }
                    prev_ma_index = index;
                }
            }
            prev = curr;
            curr = &curr_node.next;
            index += 1;
        }

        if min_distance == i32::MAX {
            vec![-1, -1]
        } else {
            vec![min_distance, prev_ma_index - first_ma_index]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
