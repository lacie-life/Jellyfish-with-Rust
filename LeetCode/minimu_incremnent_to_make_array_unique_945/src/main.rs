struct Solution;

impl Solution {
    pub fn min_increment_for_unique(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut min_available = 0;

        for &num in &nums {
            ans += (min_available - num).max(0);
            min_available = min_available.max(num) + 1;
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
    let nums = vec![3, 2, 1, 2, 1, 7];
    let result = Solution::min_increment_for_unique(nums);
    println!("{}", result);
}