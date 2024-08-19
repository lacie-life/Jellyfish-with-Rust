struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut n = n;
        let mut steps = 0;
        let mut factor = 2;

        while n > 1 {
            while n % factor == 0 {
                steps += factor;
                n /= factor;
            }
            factor += 1;
        }

        steps
    }
}

fn main() {
    println!("Hello, world!");

    let n = 3;
    let result = Solution::min_steps(n);
    println!("Result: {}", result);

    let n = 4;
    let result = Solution::min_steps(n);
    println!("Result: {}", result);
}
