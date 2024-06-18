struct Solution(Vec<i32>, Vec<i32>, Vec<i32>);

impl Solution {
    pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
        let mut jobs: Vec<(i32, i32)> = difficulty.into_iter().zip(profit.into_iter()).collect();
        jobs.sort_unstable();
        let mut max_profit_so_far = 0;
        for job in &mut jobs {
            max_profit_so_far = max_profit_so_far.max(job.1);
            job.1 = max_profit_so_far;
        }
        let mut worker = worker;
        worker.sort_unstable();
        let (mut i, mut total_profit) = (0, 0);
        for ability in worker {
            while i < jobs.len() && ability >= jobs[i].0 {
                i += 1;
            }
            if i > 0 {
                total_profit += jobs[i - 1].1;
            }
        }
        total_profit
    }
}

fn main() {
    println!("Hello, world!");

    let difficulty = vec![2, 4, 6, 8, 10];
    let profit = vec![10, 20, 30, 40, 50];
    let worker = vec![4, 5, 6, 7];
    let result = Solution::max_profit_assignment(difficulty, profit, worker);
    println!("Result: {}", result);

    let difficulty = vec![85, 47, 57];
    let profit = vec![24, 66, 99];
    let worker = vec![40, 25, 25];
    let result = Solution::max_profit_assignment(difficulty, profit, worker);
    println!("Result: {}", result);

    let difficulty = vec![68, 35, 52, 47, 86];
    let profit = vec![67, 17, 1, 81, 3];
    let worker = vec![92, 10, 85, 84, 82];
    let result = Solution::max_profit_assignment(difficulty, profit, worker);
    println!("Result: {}", result);
}
