struct Solution (String);

impl Solution {
    pub fn wonderful_substrings(word: String) -> i64 {
        let mut count = vec![0; 1024];
        count[0] = 1;
        let mut result = 0;
        let mut mask = 0;
        for c in word.chars() {
            mask ^= 1 << (c as u8 - b'a');
            result += count[mask];
            for i in 0..10 {
                result += count[mask ^ (1 << i)];
            }
            count[mask] += 1;
        }
        result
    }
}

fn main() {
    println!("Hello, world!");

    let word = "aba".to_string();
    let result = Solution::wonderful_substrings(word);
    println!("result: {}", result);

    let word = "aabb".to_string();
    let result = Solution::wonderful_substrings(word);
    println!("result: {}", result);
}
