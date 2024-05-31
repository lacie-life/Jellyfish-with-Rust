struct Solution(Vec<i32>);

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut xor = 0;
        for &num in nums.iter() {
            xor ^= num;
        }

        let mut diff = xor & (-xor);
        let mut rets = vec![0, 0]; // this vector stores the two numbers we will return
        for &num in nums.iter() {
            if (num & diff) == 0 {
                rets[0] ^= num;
            } else {
                rets[1] ^= num;
            }
        }
        rets
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 2, 1, 3, 2, 5];
    let result = Solution::single_number(nums);
    println!("{:?}", result);
}
