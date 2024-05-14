struct Solution (Vec<Vec<i32>>);

impl Solution {
    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                ans = ans.max(Self::dfs(&mut grid.clone(), i as i32, j as i32));
            }
        }

        ans
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> i32 {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;

        if i < 0 || j < 0 || i == rows || j == cols {
            return 0;
        }

        if grid[i as usize][j as usize] == 0 {
            return 0;
        }

        let gold = grid[i as usize][j as usize];
        grid[i as usize][j as usize] = 0; // Mark as visited.

        let max_path = vec![
            Self::dfs(grid, i + 1, j),
            Self::dfs(grid, i - 1, j),
            Self::dfs(grid, i, j + 1),
            Self::dfs(grid, i, j - 1),
        ]
            .into_iter()
            .max()
            .unwrap();

        grid[i as usize][j as usize] = gold;
        gold + max_path
    }
}

fn main() {
    let grid = vec![
        vec![0, 6, 0],
        vec![5, 8, 7],
        vec![0, 9, 0],
    ];

    let ans = Solution::get_maximum_gold(grid);
    println!("{}", ans);
}


