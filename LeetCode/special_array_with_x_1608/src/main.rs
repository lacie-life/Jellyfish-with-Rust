struct Solution(Vec<i32>);

impl Solution {
    pub fn special_array(nums: Vec<i32>) -> i32 {

        let mut nums = nums;
        nums.sort();
        let n = nums.len() as i32;

        for i in 0..=n {
            if i == n {
                return -1;
            }
            if i == 0 {
                if nums[0] >= n {
                    return n;
                }
            } else {
                if nums[i as usize] >= n - i && nums[(i - 1) as usize] < n - i {
                    return n - i;
                }
            }
        }

        -1

    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![3, 5];
    let result = Solution::special_array(nums);
    println!("Result: {}", result);
}
