struct Solution(Vec<i32>);

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0; // If there are 4 or fewer elements, no difference after changes.
        }

        nums.sort(); // Sort the array to simplify finding min and max differences.

        let n = nums.len();
        let mut min_diff = i32::MAX;

        // Consider all cases of changing up to three elements.
        for i in 0..=3 {
            let diff = nums[n - 4 + i] - nums[i];
            min_diff = min_diff.min(diff);
        }

        min_diff
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![5, 3, 2, 4];
    let result = Solution::min_difference(nums);
    println!("Result: {}", result);
}
