use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut freq_map = HashMap::new();
        // Count the frequency of each number
        for &num in &nums {
            *freq_map.entry(num).or_insert(0) += 1;
        }
        let mut nums = nums;
        // Sort the nums vector based on the frequency, then by value decreasingly if frequencies are equal
        nums.sort_unstable_by(|a, b| {
            let freq_a = freq_map.get(a).unwrap();
            let freq_b = freq_map.get(b).unwrap();
            freq_a.cmp(freq_b).then_with(|| b.cmp(a))
        });
        nums
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 1, 2, 2, 2, 3];
    let result = Solution::frequency_sort(nums);
    println!("{:?}", result);
}
