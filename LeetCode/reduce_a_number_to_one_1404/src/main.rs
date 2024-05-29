struct Solution(String);

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut s = s;
        let mut ans = 0;

        // All the trailing 0s can be popped by 1 step.
        while s.ends_with('0') {
            s.pop();
            ans += 1;
        }

        if s == "1" {
            return ans;
        }

        // `s` is now odd, so add 1 to `s` and cost 1 step.
        ans += 1;

        // All the 1s will become 0s and can be popped by 1 step.
        // All the 0s will become 1s and can be popped by 2 steps (adding 1 then
        // dividing by 2).
        for c in s.chars() {
            ans += if c == '1' { 1 } else { 2 };
        }

        ans
    }
}


fn main() {
    println!("Hello, world!");

    let s = "1101".to_string();
    let result = Solution::num_steps(s);
    println!("Result: {}", result);

    let s = "10".to_string();
    let result = Solution::num_steps(s);
    println!("Result: {}", result);
}
