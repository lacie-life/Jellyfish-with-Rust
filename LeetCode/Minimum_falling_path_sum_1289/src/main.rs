struct Solution (Vec<Vec<i32>>);

impl Solution {
    pub fn min_falling_path_sum(arr: Vec<Vec<i32>>) -> i32 {
        let m = arr.len();
        let n = arr[0].len();
        let mut dp = vec![vec![0; n]; m];

        for j in 0..n {
            dp[0][j] = arr[0][j];
        }

        // Calculate the minimum falling path sum for each cell in the matrix
        for i in 1..m {
            let last_row = dp[i - 1].clone();
            let last_row_min_idx = last_row.iter().position(|&x| x == *last_row.iter().min().unwrap()).unwrap();
            let last_row_min = last_row[last_row_min_idx];
            let mut last_row_sorted = last_row.clone();
            last_row_sorted.sort();
            let last_row_2min = last_row_sorted[1];

            for j in 0..n {
                if j == last_row_min_idx {
                    dp[i][j] = arr[i][j] + last_row_2min;
                } else {
                    dp[i][j] = arr[i][j] + last_row_min;
                }
            }
        }

        *dp[m - 1].iter().min().unwrap()
    }
}

fn main() {
    println!("Hello, world!");

    let matrix = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let result = Solution::min_falling_path_sum(matrix);
    println!("result = {}", result);

    let matrix = vec![vec![7]];
    let result = Solution::min_falling_path_sum(matrix);
    println!("result = {}", result);

}
