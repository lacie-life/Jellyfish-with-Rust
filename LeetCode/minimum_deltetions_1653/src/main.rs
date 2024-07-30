use std::cmp::min;
struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let mut dp = 0;
        let mut count_b = 0;

        for c in s.chars() {
            if c == 'a' {
                dp = min(dp + 1, count_b);
            } else {
                count_b += 1;
            }
        }

        dp
    }
}

fn main() {
    println!("Hello, world!");

    let s = "aababbab".to_string();
    let result = Solution::minimum_deletions(s);
    println!("Result: {}", result);

    let s = "bbaaaaabb".to_string();
    let result = Solution::minimum_deletions(s);
    println!("Result: {}", result);
}
