struct Solution (String, char);

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let mut result = String::new();
        let mut found = false;
        for c in word.chars() {
            if c == ch {
                found = true;
                result.push(c);
                break;
            }
            result.push(c);
        }
        if found {
            result = result.chars().rev().collect();
        }
        result.push_str(&word[result.len()..]);
        result
    }
}


fn main() {
    println!("Hello, world!");

    let word = "abcdefd".to_string();
    let ch = 'd';
    let result = Solution::reverse_prefix(word, ch);
    println!("result: {}", result);
}
