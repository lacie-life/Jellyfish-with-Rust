struct Solution(Vec<i32>, i32);

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let mut nice_subarrays = 0;
        let mut odd_count = 0;
        let mut count = vec![0; nums.len() + 1];
        count[0] = 1;

        for right in 0..nums.len() {
            if nums[right] % 2 == 1 {
                odd_count += 1;
            }
            count[odd_count as usize] += 1;
            if odd_count >= k {
                nice_subarrays += count[(odd_count - k) as usize];
            }
        }

        nice_subarrays
    }
}


fn main() {
    println!("Hello, world!");

    let nums = vec![1, 1, 2, 1, 1];
    let k = 3;
    let result = Solution::number_of_subarrays(nums, k);
    println!("Result: {}", result);

    let nums = vec![2, 4, 6];
    let k = 1;
    let result = Solution::number_of_subarrays(nums, k);
    println!("Result: {}", result);
}
