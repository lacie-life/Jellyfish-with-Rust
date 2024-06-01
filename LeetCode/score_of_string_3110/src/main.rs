pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        let mut ans = 0;
        let bytes = s.as_bytes();

        for i in 1..bytes.len() {
            ans += (bytes[i] as i32 - bytes[i - 1] as i32).abs();
        }

        ans
    }
}

fn main() {

    println!("Hello, world!");
    let s = "abc".to_string();
    let result = Solution::score_of_string(s);
    println!("Score of the string: {}", result);
}