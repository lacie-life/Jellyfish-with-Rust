struct Solution;

impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums[nums.len() - 1] - nums[0];

        while left < right {
            let mid = (left + right) / 2;
            if Self::count_pairs(&nums, mid) < k {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }

    fn count_pairs(nums: &Vec<i32>, mid: i32) -> i32 {
        let mut count = 0;
        let mut j = 0;

        for i in 0..nums.len() {
            while j < nums.len() && nums[j] - nums[i] <= mid {
                j += 1;
            }
            count += j - i - 1;
        }

        count as i32
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 3, 1];
    let k = 1;
    let result = Solution::smallest_distance_pair(nums, k);
    println!("result = {}", result);

    let nums = vec![1, 1, 1];
    let k = 2;
    let result = Solution::smallest_distance_pair(nums, k);
    println!("result = {}", result);

    let nums = vec![1, 6, 1];
    let k = 3;
    let result = Solution::smallest_distance_pair(nums, k);
    println!("result = {}", result);
}
