struct Solution(Vec<i32>);

impl Solution {
    pub fn find_relative_ranks (score: Vec<i32>) -> Vec<String> {
        let mut score = score;
        let mut rank = vec![0; score.len()];
        for i in 0..score.len() {
            rank[i] = i;
        }
        rank.sort_by_key(|&i| -score[i]);
        let mut res = vec![String::new(); score.len()];
        for i in 0..score.len() {
            res[rank[i]] = match i {
                0 => "Gold Medal".to_string(),
                1 => "Silver Medal".to_string(),
                2 => "Bronze Medal".to_string(),
                _ => (i + 1).to_string(),
            };
        }
        res
    }
}

fn main() {
    println!("Hello, world!");
}
