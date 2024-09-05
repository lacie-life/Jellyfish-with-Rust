struct Solution;

impl Solution {
    pub fn missing_rolls(rolls: Vec<i32>, mean: i32, n: i32) -> Vec<i32> {
        let m = rolls.len() as i32;
        let total_sum = mean * (n + m);
        let sum_of_rolls: i32 = rolls.iter().sum();
        let missing_sum = total_sum - sum_of_rolls;

        if missing_sum < n || missing_sum > 6 * n {
            return vec![];
        }

        let mut result = vec![missing_sum / n; n as usize];
        let mut remainder = missing_sum % n;

        for i in 0..remainder as usize {
            result[i] += 1;
        }

        result
    }
}

fn main() {
    println!("Hello, world!");

    let rolls = vec![3, 2, 4, 3];
    let mean = 4;
    let n = 2;
    let result = Solution::missing_rolls(rolls, mean, n);
    println!("{:?}", result);
}
