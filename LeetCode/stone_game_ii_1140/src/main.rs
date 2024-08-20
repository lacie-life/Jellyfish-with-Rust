use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();
        let mut memo = HashMap::new();
        let mut suffix_sum = vec![0; n + 1];

        for i in (0..n).rev() {
            suffix_sum[i] = suffix_sum[i + 1] + piles[i];
        }

        fn dp(i: usize, m: usize, piles: &Vec<i32>, suffix_sum: &Vec<i32>, memo: &mut HashMap<(usize, usize), i32>) -> i32 {
            if i >= piles.len() {
                return 0;
            }
            if 2 * m >= piles.len() - i {
                return suffix_sum[i];
            }
            if let Some(&res) = memo.get(&(i, m)) {
                return res;
            }
            let mut max_stones = 0;
            for x in 1..=2 * m {
                max_stones = max_stones.max(suffix_sum[i] - dp(i + x, m.max(x), piles, suffix_sum, memo));
            }
            memo.insert((i, m), max_stones);
            max_stones
        }

        dp(0, 1, &piles, &suffix_sum, &mut memo)
    }
}

fn main() {
    println!("Hello, world!");

    let piles = vec![2, 7, 9, 4, 4];
    let result = Solution::stone_game_ii(piles);
    println!("result = {}", result);

    let piles = vec![1, 2, 3, 4, 5, 100];
    let result = Solution::stone_game_ii(piles);
    println!("result = {}", result);
}
