use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

struct Solution;

impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let n = n as usize;
        let mut ugly_numbers = vec![1; n];
        let (mut i2, mut i3, mut i5) = (0, 0, 0);

        for i in 1..n {
            let next_ugly = std::cmp::min(
                ugly_numbers[i2] * 2,
                std::cmp::min(ugly_numbers[i3] * 3, ugly_numbers[i5] * 5),
            );
            ugly_numbers[i] = next_ugly;

            if next_ugly == ugly_numbers[i2] * 2 {
                i2 += 1;
            }
            if next_ugly == ugly_numbers[i3] * 3 {
                i3 += 1;
            }
            if next_ugly == ugly_numbers[i5] * 5 {
                i5 += 1;
            }
        }

        ugly_numbers[n - 1]
    }
}

fn main() {
    println!("Hello, world!");

    let n = 10;
    let result = Solution::nth_ugly_number(n);
    println!("The {}th ugly number is: {}", n, result);
}
