use std::collections::HashSet;

struct Solution(String, Vec<String>);

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        let dict: HashSet<_> = word_dict.into_iter().collect();
        let mut dp = vec![vec![]; s.len() + 1];
        dp[0].push("".to_string());

        for i in 1..=s.len() {
            for j in 0..i {
                if !dp[j].is_empty() && dict.contains(&s[j..i]) {
                    let sentences = dp[j].clone();
                    for sentence in sentences {
                        dp[i].push(if sentence.is_empty() {
                            s[j..i].to_string()
                        } else {
                            format!("{} {}", sentence, &s[j..i])
                        });
                    }
                }
            }
        }

        dp[s.len()].clone()
    }
}

fn main() {
    println!("Hello, world!");

    let s = "catsanddog".to_string();
    let word_dict = vec!["cat".to_string(), "cats".to_string(), "and".to_string(), "sand".to_string(), "dog".to_string()];
    let result = Solution::word_break(s, word_dict);
    println!("{:?}", result);
}
