use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let mut prefix = 0;
        let mut prefix_to_index = HashMap::new();
        prefix_to_index.insert(0, -1);

        for (i, &num) in nums.iter().enumerate() {
            prefix += num;
            if k != 0 {
                prefix %= k;
            }
            if let Some(&prev_index) = prefix_to_index.get(&prefix) {
                if (i as i32) - prev_index > 1 {
                    return true;
                }
            } else {
                prefix_to_index.insert(prefix, i as i32);
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
    let nums = vec![23, 2, 4, 6, 7];
    let k = 6;
    let result = Solution::check_subarray_sum(nums, k);
    println!("Result: {}", result);
}
