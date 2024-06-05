struct Solution;

impl Solution {
    pub fn common_chars(a: Vec<String>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut common_count = vec![i32::MAX; 26]; // Initialize with INT_MAX

        // Count frequency of each character in each string and update common_count
        for s in &a {
            let mut count = vec![0; 26];
            for c in s.chars() {
                count[c as usize - 'a' as usize] += 1;
            }
            for i in 0..26 {
                common_count[i] = common_count[i].min(count[i]);
            }
        }

        // Collect common characters based on the common_count
        for (i, &count) in common_count.iter().enumerate() {
            for _ in 0..count {
                ans.push(((i as u8 + b'a') as char).to_string());
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
    let a = vec!["bella".to_string(), "label".to_string(), "roller".to_string()];
    let result = Solution::common_chars(a);
    println!("{:?}", result);
}
