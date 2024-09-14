struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut max_and = 0;
        let mut max_length = 0;
        let mut current_length = 0;

        // Find the maximum bitwise AND value
        for &num in &nums {
            max_and = max_and.max(num);
        }

        // Find the longest subarray with the maximum bitwise AND value
        for &num in &nums {
            if num == max_and {
                current_length += 1;
                max_length = max_length.max(current_length);
            } else {
                current_length = 0;
            }
        }

        max_length
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 1, 0, 0, 1, 1, 1, 0, 1];
    let result = Solution::longest_subarray(nums);
    println!("Result: {}", result);
}
