use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn kth_distinct(arr: Vec<String>, k: i32) -> String {
        let mut count_map = HashMap::new();

        for s in &arr {
            *count_map.entry(s).or_insert(0) += 1;
        }

        let mut distinct_count = 0;
        for s in &arr {
            if count_map[s] == 1 {
                distinct_count += 1;
                if distinct_count == k {
                    return s.clone();
                }
            }
        }

        "".to_string()
    }
}

fn main() {
    println!("Hello, world!");
    let arr = vec!["d".to_string(), "b".to_string(), "c".to_string(), "b".to_string(), "c".to_string(), "a".to_string()];
    let k = 2;
    let result = Solution::kth_distinct(arr, k);
    println!("Result: {}", result);
}
