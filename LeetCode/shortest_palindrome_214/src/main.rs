struct Solution;

impl Solution {
    pub fn shortest_palindrome(s: String) -> String {
        let rev_s: String = s.chars().rev().collect();
        let new_s = format!("{}#{}", s, rev_s);
        let lps = Self::compute_lps(&new_s);
        let add_len = s.len() - lps[new_s.len() - 1];
        let add_str: String = rev_s.chars().take(add_len).collect();
        format!("{}{}", add_str, s)
    }

    fn compute_lps(s: &str) -> Vec<usize> {
        let mut lps = vec![0; s.len()];
        let mut len = 0;
        let mut i = 1;
        while i < s.len() {
            if s.as_bytes()[i] == s.as_bytes()[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }
        lps
    }
}


fn main() {
    println!("Hello, world!");

    let s = "aacecaaa".to_string();
    let result = Solution::shortest_palindrome(s);
    println!("result = {}", result);
}
