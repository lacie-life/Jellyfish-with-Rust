struct Solution;

impl Solution {
    pub fn spiral_matrix_iii(rows: i32, cols: i32, r_start: i32, c_start: i32) -> Vec<Vec<i32>> {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut result = Vec::with_capacity((rows * cols) as usize);
        let (mut r, mut c) = (r_start, c_start);
        let mut dir = 0;
        let mut steps = 1;

        result.push(vec![r, c]);

        while result.len() < (rows * cols) as usize {
            for _ in 0..2 {
                for _ in 0..steps {
                    r += directions[dir].0;
                    c += directions[dir].1;
                    if r >= 0 && r < rows && c >= 0 && c < cols {
                        result.push(vec![r, c]);
                    }
                }
                dir = (dir + 1) % 4;
            }
            steps += 1;
        }

        result
    }
}

fn main() {
    println!("Hello, world!");

    let rows = 1;
    let cols = 4;
    let r_start = 0;
    let c_start = 0;
    let result = Solution::spiral_matrix_iii(rows, cols, r_start, c_start);
    println!("{:?}", result);

    let rows = 5;
    let cols = 6;
    let r_start = 1;
    let c_start = 4;
    let result = Solution::spiral_matrix_iii(rows, cols, r_start, c_start);
    println!("{:?}", result);
}
