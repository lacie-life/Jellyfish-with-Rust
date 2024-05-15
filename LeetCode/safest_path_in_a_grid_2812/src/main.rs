use std::collections::VecDeque;
use std::cmp;

struct Solution (Vec<Vec<i32>>);

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let dist_to_thief = Self::get_dist_to_thief(&grid);
        let mut l = 0;
        let mut r = grid.len() * 2;

        while l < r {
            let m = (l + r) / 2;
            if Self::has_valid_path(&dist_to_thief, m) {
                l = m + 1;
            } else {
                r = m;
            }
        }

        (l - 1) as i32
    }

    fn has_valid_path(dist_to_thief: &Vec<Vec<i32>>, safeness: usize) -> bool {
        let n = dist_to_thief.len();
        if dist_to_thief[0][0] < safeness as i32 {
            return false;
        }

        let mut q = VecDeque::new();
        q.push_back((0, 0));
        let mut seen = vec![vec![false; n]; n];
        seen[0][0] = true;

        while let Some((i, j)) = q.pop_front() {
            if dist_to_thief[i][j] < safeness as i32 {
                continue;
            }
            if i == n - 1 && j == n - 1 {
                return true;
            }
            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let x = (i as i32 + dx) as usize;
                let y = (j as i32 + dy) as usize;
                if x >= n || y >= n {
                    continue;
                }
                if seen[x][y] {
                    continue;
                }
                q.push_back((x, y));
                seen[x][y] = true;
            }
        }

        false
    }

    fn get_dist_to_thief(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut dist_to_thief = vec![vec![0; n]; n];
        let mut q = VecDeque::new();
        let mut seen = vec![vec![false; n]; n];

        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    q.push_back((i, j));
                    seen[i][j] = true;
                }
            }
        }

        let dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for dist in 0.. {
            for sz in 0..q.len() {
                if let Some((i, j)) = q.pop_front() {
                    dist_to_thief[i][j] = dist;
                    for &(dx, dy) in &dirs {
                        let x = (i as i32 + dx) as usize;
                        let y = (j as i32 + dy) as usize;
                        if x >= n || y >= n {
                            continue;
                        }
                        if seen[x][y] {
                            continue;
                        }
                        q.push_back((x, y));
                        seen[x][y] = true;
                    }
                }
            }
            if q.is_empty() {
                break;
            }
        }

        dist_to_thief
    }
}

fn main() {
    println!("Hello, world!");

    let grid = vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 1, 0, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0]
    ];

    let result = Solution::maximum_safeness_factor(grid);
    println!("Result: {}", result);
}

