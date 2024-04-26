struct Solution (Vec<Vec<i32>>);

use std::cmp::{max, min};

impl Solution {
    fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        let n = matrix.len();
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            dp[0][i] = matrix[0][i];
        }

        for i in 1..n {
            for j in 0..n {
                let mut min_prev = dp[i - 1][j];
                if j > 0 {
                    min_prev = min(min_prev, dp[i - 1][j - 1]);
                }
                if j < n - 1 {
                    min_prev = min(min_prev, dp[i - 1][j + 1]);
                }
                dp[i][j] = matrix[i][j] + min_prev;
            }
        }

        *dp[n - 1].iter().min().unwrap()
    }
}

fn main() {
    println!("Hello, world!");

    let arr = vec![vec![2,1,3], vec![6,5,4], vec![7,8,9]];
    let result = Solution::min_falling_path_sum(arr);
    println!("Result: {}", result);

    let arr = vec![vec![-19,57], vec![-40,-5]];
    let result = Solution::min_falling_path_sum(arr);
    println!("Result: {}", result);
}
