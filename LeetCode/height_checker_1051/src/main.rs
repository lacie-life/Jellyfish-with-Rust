struct Solution(Vec<i32>);

impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sorted = heights.clone();
        sorted.sort();
        heights.iter().zip(sorted.iter()).filter(|(a, b)| a != b).count() as i32
    }
}

fn main() {
    println!("Hello, world!");
    let heights = vec![1, 1, 4, 2, 1, 3];
    let result = Solution::height_checker(heights);
    println!("Result: {}", result);
}
