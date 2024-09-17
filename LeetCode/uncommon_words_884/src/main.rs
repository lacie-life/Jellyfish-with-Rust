use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut word_count = HashMap::new();

        // Split sentences into words and count occurrences
        for word in s1.split_whitespace().chain(s2.split_whitespace()) {
            *word_count.entry(word).or_insert(0) += 1;
        }

        // Collect words that appear exactly once
        word_count.into_iter()
            .filter(|&(_, count)| count == 1)
            .map(|(word, _)| word.to_string())
            .collect()
    }
}

fn main() {
    println!("Hello, world!");

    let s1 = "this apple is sweet".to_string();
    let s2 = "this apple is sour".to_string();
    let result = Solution::uncommon_from_sentences(s1, s2);
    println!("{:?}", result);
}
