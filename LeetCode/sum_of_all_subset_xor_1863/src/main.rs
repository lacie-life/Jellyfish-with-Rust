struct Solution(Vec<i32>);

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        Self::xor_all_subsets(0, 0, &nums)
    }

    fn xor_all_subsets(idx: usize, curr_xor: i32, nums: &[i32]) -> i32 {
        if idx == nums.len() {
            return curr_xor;
        }
        Self::xor_all_subsets(idx + 1, curr_xor ^ nums[idx], nums) + Self::xor_all_subsets(idx + 1, curr_xor, nums)
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 3];
    let result = Solution::subset_xor_sum(nums);
    println!("Result: {}", result);

    let nums = vec![5, 1, 6];
    let result = Solution::subset_xor_sum(nums);
    println!("Result: {}", result);
}
