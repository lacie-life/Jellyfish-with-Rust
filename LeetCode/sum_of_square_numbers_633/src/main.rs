struct Solution(i32);

impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let c = c as i64; // Some test cases are too large for i32
        let mut a: i64 = 0;
        let mut b: i64 = (c as f64).sqrt() as i64;
        while a <= b {
            let sum = a*a + b*b;
            if sum == c {
                return true;
            } else if sum < c {
                a += 1;
            } else {
                b -= 1;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");

    let c = 5;
    let result = Solution::judge_square_sum(c);
    println!("Result: {}", result);
}
