use std::collections::VecDeque;

struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut ans = 1;
        let mut min_q = VecDeque::new();
        let mut max_q = VecDeque::new();

        let mut l = 0;
        for r in 0..nums.len() {
            while let Some(&back) = min_q.back() {
                if back > nums[r] {
                    min_q.pop_back();
                } else {
                    break;
                }
            }
            min_q.push_back(nums[r]);

            while let Some(&back) = max_q.back() {
                if back < nums[r] {
                    max_q.pop_back();
                } else {
                    break;
                }
            }
            max_q.push_back(nums[r]);

            while max_q.front().unwrap() - min_q.front().unwrap() > limit {
                if min_q.front().unwrap() == &nums[l] {
                    min_q.pop_front();
                }
                if max_q.front().unwrap() == &nums[l] {
                    max_q.pop_front();
                }
                l += 1;
            }

            ans = ans.max(r - l + 1);
        }

        ans as i32
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![8,2,4,7];
    let limit = 4;
    let result = Solution::longest_subarray(nums, limit);
    println!("Result: {}", result);

    let nums = vec![10,1,2,4,7,2];
    let limit = 5;
    let result = Solution::longest_subarray(nums, limit);
    println!("Result: {}", result);

    let nums = vec![4,2,2,2,4,4,2,2];
    let limit = 0;
    let result = Solution::longest_subarray(nums, limit);
    println!("Result: {}", result);
}
