struct Solution(String, String, i32);

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut max_len = 0;
        let mut cost = 0;
        let mut left = 0;
        for right in 0..s.len() {
            cost += (s[right] as i32 - t[right] as i32).abs();
            while cost > max_cost {
                cost -= (s[left] as i32 - t[left] as i32).abs();
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }
        max_len as i32
    }
}

fn main() {
    println!("Hello, world!");

    let s = "abcd".to_string();
    let t = "bcdf".to_string();
    let max_cost = 3;
    let result = Solution::equal_substring(s, t, max_cost);
    println!("result = {}", result);

}
