use std::cmp::{max, min};

struct Solution (String, i32);

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = vec![0; 26];

        for c in s.chars() {
            let i = (c as u8 - 'a' as u8) as usize;
            dp[i] = 1 + Solution::get_max_reachable(&dp, i, k);
        }

        *dp.iter().max().unwrap_or(&0)
    }

    fn get_max_reachable(dp: &[i32], i: usize, k: i32) -> i32 {
        let first = max(0, i as i32 - k) as usize;
        let last = min(25, i as i32 + k) as usize;
        let mut max_reachable = 0;
        for j in first..=last {
            max_reachable = max(max_reachable, dp[j]);
        }
        max_reachable
    }
}

fn main() {
    println!("Hello, world!");

    let s = "abcd".to_string();
    let k = 3;
    let result = Solution::longest_ideal_string(s, k);
    println!("Result: {}", result);

    let s = "acfgbd".to_string();
    let k = 2;
    let result = Solution::longest_ideal_string(s, k);
    println!("Result: {}", result);
}
