struct Solution(Vec<i32>, i32);

impl Solution {
    pub fn min_k_bit_flips(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut flips = vec![0; n];
        let mut flip = 0;
        let mut res = 0;

        for i in 0..n {
            if i >= k as usize {
                flip ^= flips[i - k as usize];
            }
            if flip % 2 == nums[i] {
                if i + k as usize > n {
                    return -1;
                }
                flip ^= 1;
                flips[i] = 1;
                res += 1;
            }
        }
        res
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![0, 1, 0];
    let k = 1;
    let result = Solution::min_k_bit_flips(nums, k);
    println!("Result: {}", result);

    let nums = vec![1, 1, 0];
    let k = 2;
    let result = Solution::min_k_bit_flips(nums, k);
    println!("Result: {}", result);

    let nums = vec![0, 0, 0, 1, 0, 1, 1, 0];
    let k = 3;
    let result = Solution::min_k_bit_flips(nums, k);
    println!("Result: {}", result);
}
