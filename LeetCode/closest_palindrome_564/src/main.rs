struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let length = n.len();
        let num = n.parse::<i64>().unwrap();

        // Edge cases
        if num <= 10 {
            return (num - 1).to_string();
        }
        if num == 11 {
            return "9".to_string();
        }
        if num == 100 || (num % 10 == 0 && num == 10i64.pow((length - 1) as u32)) {
            return (num - 1).to_string();
        }

        // Generate candidates
        let mut candidates = vec![
            10i64.pow(length as u32) + 1, // 100...001
            10i64.pow((length - 1) as u32) - 1, // 999...999
        ];

        let prefix = &n[..(length + 1) / 2];
        let prefix_num = prefix.parse::<i64>().unwrap();

        for i in -1..=1 {
            let new_prefix = (prefix_num + i).to_string();
            let mut candidate = new_prefix.clone();
            let rev = new_prefix.chars().rev().collect::<String>();
            if length % 2 == 0 {
                candidate.push_str(&rev);
            } else {
                candidate.push_str(&rev[1..]);
            }
            candidates.push(candidate.parse::<i64>().unwrap());
        }

        // Find the closest palindrome
        candidates.retain(|&x| x != num);
        candidates.sort_by_key(|&x| ((num - x).abs(), x));

        candidates[0].to_string()
    }
}

fn main() {
    println!("Hello, world!");

    let n = "123".to_string();
    let result = Solution::nearest_palindromic(n);
    println!("result = {}", result);

    let n = "1234".to_string();
    let result = Solution::nearest_palindromic(n);
    println!("result = {}", result);
}
