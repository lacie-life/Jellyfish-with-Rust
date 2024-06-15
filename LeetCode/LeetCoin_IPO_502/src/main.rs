use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn find_maximized_capital(k: i32, w: i32, profits: Vec<i32>, capital: Vec<i32>) -> i32 {
        let mut projects: Vec<_> = capital.into_iter().zip(profits.into_iter()).collect();
        projects.sort_unstable();
        let mut capital = w;
        let mut heap = BinaryHeap::new();
        let mut i = 0;
        for _ in 0..k {
            while i < projects.len() && projects[i].0 <= capital {
                heap.push(projects[i].1);
                i += 1;
            }
            if let Some(profit) = heap.pop() {
                capital += profit;
            } else {
                break;
            }
        }
        capital
    }
}


fn main() {
    println!("Hello, world!");
    let k = 2;
    let w = 0;
    let profits = vec![1, 2, 3];
    let capital = vec![0, 1, 1];
    let result = Solution::find_maximized_capital(k, w, profits, capital);
    println!("result = {}", result);
}
