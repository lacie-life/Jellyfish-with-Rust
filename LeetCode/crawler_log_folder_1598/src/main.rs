struct Solution(Vec<String>);

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut depth = 0;
        for log in logs {
            match log.as_str() {
                "../" => if depth > 0 { depth -= 1; },
                "./" => {},
                _ => depth += 1,
            }
        }
        depth
    }
}

fn main() {
    println!("Hello, world!");

    let logs = vec!["d1/".to_string(), "d2/".to_string(), "../".to_string(), "d21/".to_string(), "./".to_string()];
    let result = Solution::min_operations(logs);
    println!("Result: {}", result);

    let logs = vec!["d1/".to_string(), "d2/".to_string(), "./".to_string(), "d3/".to_string(), "../".to_string(), "d31/".to_string()];
    let result = Solution::min_operations(logs);
    println!("Result: {}", result);
}
