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
    pub fn spiral_matrix(m: i32, n: i32, mut head: Option<Box<ListNode>>) -> Vec<Vec<i32>> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)]; // right, down, left, up
        let mut matrix = vec![vec![-1; n as usize]; m as usize];
        let mut x = 0;
        let mut y = 0;
        let mut dir = 0;

        while let Some(node) = head {
            matrix[x][y] = node.val;
            let next_x = x as i32 + directions[dir].0;
            let next_y = y as i32 + directions[dir].1;

            if next_x < 0 || next_x >= m || next_y < 0 || next_y >= n || matrix[next_x as usize][next_y as usize] != -1 {
                dir = (dir + 1) % 4; // change direction
            }

            x = (x as i32 + directions[dir].0) as usize;
            y = (y as i32 + directions[dir].1) as usize;
            head = node.next;
        }

        matrix
    }
}

fn main() {
    println!("Hello, world!");
}
