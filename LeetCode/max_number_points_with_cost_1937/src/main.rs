struct Solution;

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>) -> i64 {
        let width = grid[0].len();
        let mut current = vec![0i64; width];
        let mut previous = vec![0i64; width];

        for level in grid.iter() {
            let mut peak = 0i64;
            // Forward sweep
            for i in 0..width {
                peak = peak.saturating_sub(1).max(previous[i]);
                current[i] = peak;
            }

            peak = 0;
            // Backward sweep
            for i in (0..width).rev() {
                peak = peak.saturating_sub(1).max(previous[i]);
                current[i] = current[i].max(peak) + level[i] as i64;
            }

            std::mem::swap(&mut previous, &mut current);
        }

        *previous.iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
    let points = vec![vec![1,2,3], vec![1,5,1], vec![3,1,1]];
    let result = Solution::max_points(points);
    println!("Result: {}", result);

    let points = vec![vec![1,5], vec![2,3], vec![4,2]];
    let result = Solution::max_points(points);
    println!("Result: {}", result);
}
