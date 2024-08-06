struct Solution;

impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut count = vec![0; 26];
        let mut ans = 0;

        for c in word.chars() {
            count[c as usize - 'a' as usize] += 1;
        }

        count.sort_by(|a, b| b.cmp(a));

        for i in 0..26 {
            ans += count[i] * (i / 8 + 1) as i32;
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
}
