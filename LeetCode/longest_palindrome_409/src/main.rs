struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut ans = 0;
        let mut count = vec![0; 128];

        for c in s.chars() {
            count[c as usize] += 1;
        }

        for &freq in &count {
            ans += if freq % 2 == 0 { freq } else { freq - 1 };
        }

        let has_odd_count = count.iter().any(|&c| c % 2 == 1);
        ans + if has_odd_count { 1 } else { 0 }
    }
}


fn main() {
    println!("Hello, world!");

    let s = "abccccdd".to_string();
    let result = Solution::longest_palindrome(s);
    println!("Result: {}", result);
}
