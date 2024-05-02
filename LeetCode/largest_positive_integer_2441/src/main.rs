struct Solution (Vec<i32>);

use std::collections::HashSet;

impl Solution {
    pub fn find_max_k(nums: Vec<i32>) -> i32 {
        let mut ans = -1;
        let mut seen = HashSet::new();

        for num in nums {
            if seen.contains(&(-num)) {
                ans = ans.max(num.abs());
            } else {
                seen.insert(num);
            }
        }

        ans
    }
}

fn main() {
    let nums = vec![-1, 2, -3, 3];
    let ans = Solution::find_max_k(nums);
    println!("{}", ans);

    let nums = vec![-1, 10, 6, 7, -7, 1];
    let ans = Solution::find_max_k(nums);
    println!("{}", ans);
}