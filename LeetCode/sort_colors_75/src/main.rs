struct Solution(Vec<i32>);

impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        if nums.len() <= 1 { return }
        let (mut zero, mut two) = (0, nums.len() - 1);
        while nums[two] == 2 {
            if let Some(t) = two.checked_sub(1) {
                two = t;
            } else { return }
        }
        while zero < nums.len() && nums[zero] == 0 {
            zero += 1;
        }
        let mut one = zero;
        while one <= two {
            if one == two {
                if nums[one] == 0 { nums.swap(zero, one) }
                return
            }
            if nums[one] == 2 {
                nums.swap(one, two);
                two -= 1;   // two cannot be 0 cause one < two
            } else if nums[one] == 0 {
                nums.swap(zero, one);
                zero += 1;
                one += 1;
            } else {
                one += 1;
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut nums = vec![2, 0, 2, 1, 1, 0];
    Solution::sort_colors(&mut nums);
    println!("{:?}", nums);
}
