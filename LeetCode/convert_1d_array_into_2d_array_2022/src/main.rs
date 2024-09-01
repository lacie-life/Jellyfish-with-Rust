struct Solution;

impl Solution {
    pub fn construct2_d_array(original: Vec<i32>, m: i32, n: i32) -> Vec<Vec<i32>> {
        if original.len() != (m * n) as usize {
            return vec![];
        }

        let mut result = vec![vec![0; n as usize]; m as usize];
        for i in 0..m as usize {
            for j in 0..n as usize {
                result[i][j] = original[i * n as usize + j];
            }
        }

        result
    }
}

fn main() {
    println!("Hello, world!");

    let original = vec![1, 2, 3, 4];
    let m = 2;
    let n = 2;
    let result = Solution::construct2_d_array(original, m, n);
    println!("{:?}", result);
}
