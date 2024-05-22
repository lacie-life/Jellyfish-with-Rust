struct Solution(String);

impl Solution {
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let mut result = Vec::new();
        let mut partition = Vec::new();
        Self::backtrack(&s, 0, &mut partition, &mut result);
        result
    }

    fn is_palindrome(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();
        for i in 0..len / 2 {
            if chars[i] != chars[len - 1 - i] {
                return false;
            }
        }
        true
    }

    fn backtrack(s: &str, start: usize, partition: &mut Vec<String>, result: &mut Vec<Vec<String>>) {
        if start == s.len() {
            result.push(partition.clone());
            return;
        }
        for i in start..s.len() {
            if Self::is_palindrome(&s[start..=i]) {
                partition.push(s[start..=i].to_string());
                Self::backtrack(s, i + 1, partition, result);
                partition.pop();
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let s = "aab".to_string();
    let result = Solution::partition(s);
    for partition in result {
        println!("{:?}", partition);
    }
}
