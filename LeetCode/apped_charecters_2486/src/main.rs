struct Solution(String, String);

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let mut i = 0;
        let t_chars: Vec<char> = t.chars().collect();

        for c in s.chars() {
            if i < t_chars.len() && c == t_chars[i] {
                i += 1;
                if i == t_chars.len() {
                    return 0;
                }
            }
        }

        (t_chars.len() - i) as i32
    }
}

fn main() {
    println!("Hello, world!");
}
