use std::cmp;

struct Solution (Vec<i32>, Vec<Vec<i32>>);

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let mut max_sum: i64 = 0;
        let mut changed_count = 0;
        let mut min_change_diff = i32::MAX;

        for &num in &nums {
            let changed_num = num ^ k;
            max_sum += cmp::max(num, changed_num) as i64;
            if changed_num > num {
                changed_count += 1;
            }
            min_change_diff = cmp::min(min_change_diff, (num - changed_num).abs());
        }

        if changed_count % 2 == 0 {
            max_sum
        } else {
            max_sum - min_change_diff as i64
        }
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![3, 8, 2];
    let k = 4;
    let edges = vec![vec![0, 1], vec![1, 2]];
    let result = Solution::maximum_value_sum(nums, k, edges);
    println!("Maximum value sum: {}", result);
}
