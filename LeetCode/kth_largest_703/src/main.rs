use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct KthLargest {
    k: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

impl KthLargest {
    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut heap = BinaryHeap::with_capacity(k as usize);
        let mut kth_largest = KthLargest {
            k: k as usize,
            heap,
        };
        for num in nums {
            kth_largest.add(num);
        }
        kth_largest
    }

    fn add(&mut self, val: i32) -> i32 {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(val));
        } else if let Some(&Reverse(top)) = self.heap.peek() {
            if val > top {
                self.heap.pop();
                self.heap.push(Reverse(val));
            }
        }
        self.heap.peek().unwrap().0
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */

fn main() {
    println!("Hello, world!");
    let mut obj = KthLargest::new(3, vec![4, 5, 8, 2]);
    println!("{}", obj.add(3));
    println!("{}", obj.add(5));
    println!("{}", obj.add(10));
    println!("{}", obj.add(9));
    println!("{}", obj.add(4));
}
