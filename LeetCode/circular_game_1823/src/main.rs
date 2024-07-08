struct Solution;

// Ref: https://en.wikipedia.org/wiki/Josephus_problem

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        // Converts back to 1-indexed.
        Self::josephus(n, k) + 1
    }

    fn josephus(n: i32, k: i32) -> i32 {
        let mut ans = 0;  // f(1, k)
        // Computes f(i, k) based on f(i - 1, k).
        for i in 2..=n {
            ans = (ans + k) % i;
        }
        ans
    }
}

fn main() {
    let n = 4;
    let k = 2;
    let result = Solution::find_the_winner(n, k);
    println!("{}", result);
}

