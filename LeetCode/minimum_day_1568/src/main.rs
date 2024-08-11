struct Solution;

impl Solution {
    pub fn min_days(mut grid: Vec<Vec<i32>>) -> i32 {
        if Self::disconnected(&grid) {
            return 0;
        }

        // Try to remove 1 land.
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    grid[i][j] = 0;
                    if Self::disconnected(&grid) {
                        return 1;
                    }
                    grid[i][j] = 1;
                }
            }
        }

        // Remove 2 lands.
        2
    }

    const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    fn disconnected(grid: &Vec<Vec<i32>>) -> bool {
        let mut islands_count = 0;
        let mut seen = vec![vec![false; grid[0].len()]; grid.len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 || seen[i][j] {
                    continue;
                }
                if islands_count > 0 {
                    return true;
                }
                islands_count += 1;
                Self::dfs(grid, i as i32, j as i32, &mut seen);
            }
        }
        islands_count != 1
    }

    fn dfs(grid: &Vec<Vec<i32>>, i: i32, j: i32, seen: &mut Vec<Vec<bool>>) {
        seen[i as usize][j as usize] = true;
        for (dx, dy) in Self::DIRS.iter() {
            let x = i + dx;
            let y = j + dy;
            if x < 0 || x >= grid.len() as i32 || y < 0 || y >= grid[0].len() as i32 {
                continue;
            }
            if grid[x as usize][y as usize] == 0 || seen[x as usize][y as usize] {
                continue;
            }
            Self::dfs(grid, x, y, seen);
        }
    }
}

fn main() {
    println!("Hello, world!");
    let grid = vec![
        vec![1, 1, 1, 0, 0],
        vec![1, 1, 0, 0, 1],
        vec![1, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1],
    ];
    let result = Solution::min_days(grid);
    println!("{}", result);
}
