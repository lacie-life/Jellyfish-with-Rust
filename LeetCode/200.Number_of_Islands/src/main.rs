
impl Solution {
    pub fn num_islands(mut grid: Vec<Vec<char>>) -> i32 {
        let mut count = 0;
        let rows = grid.len();
        let cols = grid[0].len();

        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        fn dfs(grid: &mut Vec<Vec<char>>, row: usize, col: usize) {
            if row >= grid.len() || col >= grid[0].len() || grid[row][col] != '1' {
                println!("Grid: {:?}", grid);
                return;
            }

            grid[row][col] = '2';

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
                let x = row as i32 + dx;
                let y = col as i32 + dy;
                if x >= 0 && y >= 0 && x < grid.len() as i32 && y < grid[0].len() as i32 {
                    dfs(grid, x as usize, y as usize);
                }
            }
        }

        for i in 0..rows {
            for j in 0..cols {
                if grid[i][j] == '1' {
                    dfs(&mut grid, i, j);
                    count += 1;
                }
            }
        }
        count
    }
}

struct Solution(Vec<Vec<char>>);

fn main() {
    println!("Hello, world!");

    let grid = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];
    let result = Solution::num_islands(grid);
    println!("Number of islands: {}", result);

    let grid = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];
    let result = Solution::num_islands(grid);
    println!("Number of islands: {}", result);

}
