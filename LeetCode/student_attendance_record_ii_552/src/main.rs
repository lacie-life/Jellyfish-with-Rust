struct Solution(i32);

impl Solution {
    pub fn check_record(n: i32) -> i32 {
        const KMOD: i64 = 1_000_000_007;
        // dp[i][j] := the length so far with i A's and the last letters are j L's
        let mut dp = vec![vec![0; 3]; 2];
        dp[0][0] = 1;

        for _ in 0..n {
            let prev = dp.clone();

            // Append a P.
            dp[0][0] = (prev[0][0] + prev[0][1] + prev[0][2]) % KMOD;

            // Append an L.
            dp[0][1] = prev[0][0];

            // Append an L.
            dp[0][2] = prev[0][1];

            // Append an A or append a P.
            dp[1][0] = (prev[0][0] + prev[0][1] + prev[0][2]
                + prev[1][0] + prev[1][1] + prev[1][2]) % KMOD;

            // Append an L.
            dp[1][1] = prev[1][0];

            // Append an L.
            dp[1][2] = prev[1][1];
        }

        dp.iter().flatten().fold(0, |acc, &x| (acc + x) % KMOD) as i32
    }
}


fn main() {
    println!("Hello, world!");
    let n = 2;
    let result = Solution::check_record(n);
    println!("Result: {}", result);
}
