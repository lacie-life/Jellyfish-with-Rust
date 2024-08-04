struct Solution;

impl Solution {
    pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
        let mut subarray_sums = Vec::new();
        let modulo = 1_000_000_007;

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {
                sum += nums[j];
                subarray_sums.push(sum);
            }
        }

        subarray_sums.sort_unstable();

        let mut result = 0;
        for i in (left - 1) as usize..right as usize {
            result = (result + subarray_sums[i]) % modulo;
        }

        result
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 2, 3, 4];
    let n = 4;
    let left = 1;
    let right = 5;
    let result = Solution::range_sum(nums, n, left, right);
    println!("Result: {}", result);
}
