struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, k: i32) -> i64 {
        // Maximize Happiness of Selected Elements
        let mut ans = 0;

        happiness.sort_by(|a, b| b.cmp(a)); // Sort in non-ascending order

        for i in 0..k {
            if i < happiness.len() as i32 {
                ans += happiness[i as usize].max(0); // Add happiness value if positive
                happiness.iter_mut().skip(i as usize + 1).for_each(|h| *h -= 1); // Decrement happiness of remaining children
            }
        }

        ans as i64
    }
}

fn main() {
    println!("Hello, world!");
}
