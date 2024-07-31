struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        let n = books.len();
        let mut dp = vec![i32::MAX; n + 1];
        dp[0] = 0;

        for i in 1..=n {
            let mut current_width = 0;
            let mut max_height = 0;
            for j in (1..=i).rev() {
                current_width += books[j - 1][0];
                if current_width > shelf_width {
                    break;
                }
                max_height = max_height.max(books[j - 1][1]);
                dp[i] = dp[i].min(dp[j - 1] + max_height);
            }
        }

        dp[n]
    }
}

fn main() {
    println!("Hello, world!");

    let books = vec![vec![1, 1], vec![2, 3], vec![2, 3], vec![1, 1], vec![1, 1], vec![1, 1], vec![1, 2]];
    let shelf_width = 4;
    let result = Solution::min_height_shelves(books, shelf_width);
    println!("result = {}", result);
}
