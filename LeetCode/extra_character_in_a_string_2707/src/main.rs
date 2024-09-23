struct Solution;

impl Solution {
    pub fn min_extra_char(s: String, dictionary: Vec<String>) -> i32 {
        let n = s.len();
        let mut dp = vec![n as i32; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            for word in &dictionary {
                let len = word.len();
                if i >= len && &s[i - len..i] == word {
                    dp[i] = dp[i].min(dp[i - len]);
                }
            }
            dp[i] = dp[i].min(dp[i - 1] + 1);
        }

        dp[n]
    }
}

fn main() {
    println!("Hello, world!");

    let s = "abc";
    let dictionary = vec!["ab", "bc"];
    let result = Solution::min_extra_char(s.to_string(), dictionary.iter().map(|s| s.to_string()).collect());
    println!("result = {}", result);


}
