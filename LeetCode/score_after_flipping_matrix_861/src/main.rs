struct Solution (Vec<Vec<i32>>);

impl Solution {
    pub fn matrix_score (grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut res = 0;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m {
            if grid[i][0] == 0 {
                for j in 0..n {
                    grid[i][j] = 1 - grid[i][j];
                }
            }
        }
        for j in 0..n {
            let mut cnt = 0;
            for i in 0..m {
                if grid[i][j] == 1 {
                    cnt += 1;
                }
            }
            res += std::cmp::max(cnt, m - cnt) * (1 << (n - 1 - j));
        }
        res as i32
    }
}


fn main() {
    println!("Hello, world!");

    let grid = vec![vec![0,0,1,1], vec![1,0,1,0], vec![1,1,0,0]];
    let result = Solution::matrix_score(grid);
    println!("Result: {}", result);
}
