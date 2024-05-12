struct Solution(Vec<Vec<i32>>);

impl Solution {
    pub fn largest_local(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let mut max_local = vec![vec![0; n - 2]; n - 2];

        for i in 1..n - 1 {
            for j in 1..n - 1 {
                let mut max_val = grid[i - 1][j - 1];
                for x in 0..3 {
                    for y in 0..3 {
                        max_val = max_val.max(grid[i - 1 + x][j - 1 + y]);
                    }
                }
                max_local[i - 1][j - 1] = max_val;
            }
        }

        max_local
    }
}

fn main() {
    println!("Hello, world!");

    let grid = vec![
        vec![1, 2, 3, 4, 5],
        vec![6, 7, 8, 9, 10],
        vec![11, 12, 13, 14, 15],
        vec![16, 17, 18, 19, 20],
        vec![21, 22, 23, 24, 25],
    ];


}
