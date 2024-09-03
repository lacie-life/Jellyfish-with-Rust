struct Solution;

impl Solution {
    pub fn get_lucky(s: String, k: i32) -> i32 {
        // Helper function to convert a character to its position in the alphabet
        fn char_to_pos(c: char) -> i32 {
            (c as u8 - b'a' + 1) as i32
        }

        // Helper function to sum the digits of an integer
        fn sum_of_digits(num_str: &str) -> i32 {
            num_str.chars().map(|c| c.to_digit(10).unwrap() as i32).sum()
        }

        // Convert the string to a large integer
        let mut num_str = String::new();
        for c in s.chars() {
            num_str.push_str(&char_to_pos(c).to_string());
        }

        // Perform the transformation k times
        let mut num = sum_of_digits(&num_str);
        for _ in 1..k {
            num = sum_of_digits(&num.to_string());
        }

        num
    }
}

fn main() {
    println!("Hello, world!");
    let s = "iiii".to_string();
    let k = 1;
    let result = Solution::get_lucky(s, k);
    println!("Result: {}", result);

    let s = "leetcode".to_string();
    let k = 2;
    let result = Solution::get_lucky(s, k);
    println!("Result: {}", result);
}
