struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        words.iter().fold(0, |subtotal, word| {
            if word.chars().all(|c| allowed.contains(c)) {
                subtotal + 1
            } else {
                subtotal
            }
        })
    }
}

fn main() {
    println!("Hello, world!");
    let allowed = "ab".to_string();
    let words = vec!["ad".to_string(), "bd".to_string(), "aaab".to_string(), "baa".to_string(), "badab".to_string()];
    let result = Solution::count_consistent_strings(allowed, words);
    println!("{}", result); // Output should be 2
}
