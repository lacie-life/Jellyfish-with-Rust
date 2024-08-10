struct Solution;

impl Solution {
    pub fn regions_by_slashes(grid: Vec<String>) -> i32 {
        let n = grid.len();
        let mut expanded_grid = vec![vec![0; n * 3]; n * 3];

        // Expand the grid
        for i in 0..n {
            for j in 0..n {
                match grid[i].chars().nth(j).unwrap() {
                    '/' => {
                        expanded_grid[i * 3][j * 3 + 2] = 1;
                        expanded_grid[i * 3 + 1][j * 3 + 1] = 1;
                        expanded_grid[i * 3 + 2][j * 3] = 1;
                    }
                    '\\' => {
                        expanded_grid[i * 3][j * 3] = 1;
                        expanded_grid[i * 3 + 1][j * 3 + 1] = 1;
                        expanded_grid[i * 3 + 2][j * 3 + 2] = 1;
                    }
                    _ => {}
                }
            }
        }

        // DFS to count regions
        fn dfs(grid: &mut Vec<Vec<i32>>, x: usize, y: usize) {
            if x >= grid.len() || y >= grid.len() || grid[x][y] != 0 {
                return;
            }
            grid[x][y] = 1;
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dx, dy) in directions {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && ny >= 0 {
                    dfs(grid, nx as usize, ny as usize);
                }
            }
        }

        let mut regions = 0;
        for i in 0..n * 3 {
            for j in 0..n * 3 {
                if expanded_grid[i][j] == 0 {
                    regions += 1;
                    dfs(&mut expanded_grid, i, j);
                }
            }
        }

        regions
    }
}

fn main() {
    println!("Hello, world!");
    let grid = vec![
        String::from(" /"),
        String::from("/ ")
    ];
    let result = Solution::regions_by_slashes(grid);
    println!("Result: {}", result);

    let grid = vec![
        String::from(" /"),
        String::from("  ")
    ];
    let result = Solution::regions_by_slashes(grid);
    println!("Result: {}", result);

    let grid = vec![
        String::from("/\\"),
        String::from("\\/")
    ];
    let result = Solution::regions_by_slashes(grid);
    println!("Result: {}", result);
}
