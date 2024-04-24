struct Solution(i32);

impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        if n == 0 {
            return dp[0];
        }
        dp[1] = 1;
        if n == 1 {
            return dp[1];
        }
        dp[2] = 1;
        if n == 2 {
            return dp[2];
        }
        for i in 3..=n as usize {
            dp[i] = dp[i - 1] + dp[i - 2] + dp[i - 3];
        }
        dp[n as usize]
    }
}

fn main() {
    println!("Hello, world!");

    let n = 25;
    let result = Solution::tribonacci(n);
    println!("The {}th tribonacci number is: {}", n, result);

    let n = 4;
    let result = Solution::tribonacci(n);
    println!("The {}th tribonacci number is: {}", n, result);
}
