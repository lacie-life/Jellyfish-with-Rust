struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let total_ones = nums.iter().filter(|&&x| x == 1).count();
        if total_ones == 0 {
            return 0;
        }

        let n = nums.len();
        let mut extended_nums = nums.clone();
        extended_nums.extend_from_slice(&nums);

        let mut max_ones_in_window = 0;
        let mut current_ones_in_window = 0;

        for i in 0..total_ones {
            if extended_nums[i] == 1 {
                current_ones_in_window += 1;
            }
        }

        max_ones_in_window = current_ones_in_window;

        for i in total_ones..extended_nums.len() {
            if extended_nums[i] == 1 {
                current_ones_in_window += 1;
            }
            if extended_nums[i - total_ones] == 1 {
                current_ones_in_window -= 1;
            }
            max_ones_in_window = max_ones_in_window.max(current_ones_in_window);
        }

        (total_ones - max_ones_in_window) as i32
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![0, 1, 0, 1, 1, 0, 0];
    let result = Solution::min_swaps(nums);
    println!("Result: {}", result);

    let nums = vec![0,1,1,1,0,0,1,1,0];
    let result = Solution::min_swaps(nums);
    println!("Result: {}", result);
}
