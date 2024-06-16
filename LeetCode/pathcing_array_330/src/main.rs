struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut miss = 1i64;

        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                ans += 1;
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
    let nums = vec![1, 5, 10];
    let n = 20;
    let result = Solution::min_patches(nums, n);
    println!("Result: {}", result);
}
