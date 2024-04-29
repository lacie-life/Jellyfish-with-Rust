// Note: The idea behind the solution is to find the XOR of all elements in nums and then calculate the XOR between this value and k.
// The resulting XOR represents the difference between the desired value k and the actual XOR of elements in nums.
// By counting the number of set bits (bits with value 1) in this XOR result,
// we can determine the minimum number of operations needed to achieve the desired XOR value k.

struct Solution(Vec<i32>, i32);

impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32{
        let xors = nums.iter().fold(0, |acc, &num| acc ^ num);
        (k ^ xors).count_ones() as i32
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![2, 1, 3, 4];
    let k = 1;
    let result = Solution::min_operations(nums, k);
    println!("Result: {}", result);
}
