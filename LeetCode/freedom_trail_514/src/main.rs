use std::collections::HashMap;
use std::cmp::min;

struct Solution(String, String);

impl Solution {
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut memo: HashMap<String, i32> = HashMap::new();
        Self::dfs(&ring, &key, 0, &mut memo) + key.len() as i32
    }

    fn dfs(ring: &str, key: &str, index: usize, memo: &mut HashMap<String, i32>) -> i32 {
        if index == key.len() {
            return 0;
        }

        let hash_key = format!("{}{}", ring, index);
        if let Some(&result) = memo.get(&hash_key) {
            return result;
        }

        let mut ans = i32::MAX;

        for (i, ch) in ring.chars().enumerate() {
            if ch == key.chars().nth(index).unwrap() {
                let min_rotates = min(i, ring.len() - i);
                let new_ring = format!("{}{}", &ring[i..], &ring[..i]);
                let remaining_rotates = Self::dfs(&new_ring, key, index + 1, memo);
                ans = min(ans, min_rotates as i32 + remaining_rotates);
            }
        }

        memo.insert(hash_key, ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let ring = "godding".to_string();
    let key = "gd".to_string();
    let result = Solution::find_rotate_steps(ring, key);
    println!("Result: {}", result);

    let ring = "godding".to_string();
    let key = "godding".to_string();
    let result = Solution::find_rotate_steps(ring, key);
    println!("Result: {}", result);
}
