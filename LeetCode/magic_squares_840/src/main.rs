struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut ans = 0;

        // Helper function to check if a 3x3 grid starting at (i, j) is a magic square
        let check = |i: usize, j: usize| -> i32 {
            if i + 3 > m || j + 3 > n {
                return 0;
            }
            let mut cnt = vec![0; 16];
            let mut row = vec![0; 3];
            let mut col = vec![0; 3];
            let mut a = 0;
            let mut b = 0;
            for x in i..i + 3 {
                for y in j..j + 3 {
                    let v = grid[x][y];
                    if v < 1 || v > 9 || { cnt[v as usize] += 1; cnt[v as usize] > 1 } {
                        return 0;
                    }
                    row[x - i] += v;
                    col[y - j] += v;
                    if x - i == y - j {
                        a += v;
                    }
                    if x - i + y - j == 2 {
                        b += v;
                    }
                }
            }
            if a != b {
                return 0;
            }
            for k in 0..3 {
                if row[k] != a || col[k] != a {
                    return 0;
                }
            }
            1
        };

        for i in 0..m {
            for j in 0..n {
                ans += check(i, j);
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");
    let grid = vec![
        vec![4, 3, 8, 4],
        vec![9, 5, 1, 9],
        vec![2, 7, 6, 2],
    ];
    let result = Solution::num_magic_squares_inside(grid);
    println!("Result: {}", result);

    let grid = vec![vec![8]];
    let result = Solution::num_magic_squares_inside(grid);
    println!("Result: {}", result);
}
