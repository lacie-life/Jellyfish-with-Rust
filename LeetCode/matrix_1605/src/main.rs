struct Solution;

impl Solution {
    pub fn restore_matrix(mut row_sum: Vec<i32>, mut col_sum: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![vec![0; col_sum.len()]; row_sum.len()];
        for i in 0..row_sum.len() {
            for j in 0..col_sum.len() {
                let val = row_sum[i].min(col_sum[j]);
                result[i][j] = val;
                // Update the sums after assigning the value
                row_sum[i] -= val;
                col_sum[j] -= val;
            }
        }
        result
    }
}


fn main() {
    println!("Hello, world!");

    let row_sum = vec![3, 8];
    let col_sum = vec![4, 7];
    let result = Solution::restore_matrix(row_sum, col_sum);
    println!("{:?}", result);
}
