struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, k: i32) -> i32 {
        let sum: i64 = chalk.iter().map(|&x| x as i64).sum();
        let mut k = (k as i64) % sum;

        if k == 0 {
            return 0;
        }

        for (i, &c) in chalk.iter().enumerate() {
            k -= c as i64;
            if k < 0 {
                return i as i32;
            }
        }

        unreachable!();
    }
}

fn main() {
    println!("Hello, world!");

    let chalk = vec![5, 1, 5];
    let k = 22;
    let result = Solution::chalk_replacer(chalk, k);
    println!("Result: {}", result);
}
