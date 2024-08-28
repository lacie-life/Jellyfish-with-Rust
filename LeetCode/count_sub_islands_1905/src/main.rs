struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize) {
        if i >= grid.len() || j >= grid[0].len() || grid[i][j] == 0 {
            return;
        }
        grid[i][j] = 0;
        if i > 0 { Self::dfs(grid, i - 1, j); }
        if j > 0 { Self::dfs(grid, i, j - 1); }
        if i < grid.len() - 1 { Self::dfs(grid, i + 1, j); }
        if j < grid[0].len() - 1 { Self::dfs(grid, i, j + 1); }
    }

    fn is_sub_island(grid1: &Vec<Vec<i32>>, grid2: &mut Vec<Vec<i32>>, i: usize, j: usize) -> bool {
        if i >= grid2.len() || j >= grid2[0].len() || grid2[i][j] == 0 {
            return true;
        }
        if grid1[i][j] == 0 {
            return false;
        }
        grid2[i][j] = 0;
        let mut result = true;
        if i > 0 { result &= Self::is_sub_island(grid1, grid2, i - 1, j); }
        if j > 0 { result &= Self::is_sub_island(grid1, grid2, i, j - 1); }
        if i < grid2.len() - 1 { result &= Self::is_sub_island(grid1, grid2, i + 1, j); }
        if j < grid2[0].len() - 1 { result &= Self::is_sub_island(grid1, grid2, i, j + 1); }
        result
    }

    pub fn count_sub_islands(mut grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;
        for i in 0..grid2.len() {
            for j in 0..grid2[0].len() {
                if grid2[i][j] == 1 && Self::is_sub_island(&grid1, &mut grid2, i, j) {
                    count += 1;
                    Self::dfs(&mut grid2, i, j);
                }
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");

    let grid1 = vec![vec![1,1,1,0,0], vec![0,1,1,1,1], vec![0,0,0,0,0], vec![1,0,0,0,0], vec![1,1,0,1,1]];
    let grid2 = vec![vec![1,1,1,0,0], vec![0,0,1,1,1], vec![0,1,0,0,0], vec![1,0,1,1,0], vec![0,1,0,1,0]];
    let result = Solution::count_sub_islands(grid1, grid2);
    println!("result = {}", result);
}
