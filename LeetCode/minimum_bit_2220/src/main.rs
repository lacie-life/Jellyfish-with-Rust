struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        // Perform XOR operation between start and goal
        let xor_result = start ^ goal;

        // Count the number of 1s in the binary representation of xor_result
        xor_result.count_ones() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
