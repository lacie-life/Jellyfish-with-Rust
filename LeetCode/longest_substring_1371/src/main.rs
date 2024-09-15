use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut state = 0;
        let mut state_map = HashMap::new();
        state_map.insert(0, -1); // Initial state before any character is processed

        for (i, c) in s.chars().enumerate() {
            match c {
                'a' => state ^= 1 << 0,
                'e' => state ^= 1 << 1,
                'i' => state ^= 1 << 2,
                'o' => state ^= 1 << 3,
                'u' => state ^= 1 << 4,
                _ => {}
            }

            if let Some(&first_occurrence) = state_map.get(&state) {
                max_len = max_len.max(i as i32 - first_occurrence);
            } else {
                state_map.insert(state, i as i32);
            }
        }

        max_len
    }
}

fn main() {
    println!("Hello, world!");

    let s = "eleetminicoworoep".to_string();
    let result = Solution::find_the_longest_substring(s);
    println!("Result: {}", result);
}
