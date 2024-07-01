struct Solution(Vec<i32>);

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        let mut count = 0;
        for &a in &arr {
            if a % 2 == 1 {
                count += 1;
                if count == 3 {
                    return true;
                }
            } else {
                count = 0;
            }
        }
        false
    }
}

fn main() {
    println!("Hello, world!");

    let arr = vec![1, 2, 34, 3, 4, 5, 7, 23, 12];
    let result = Solution::three_consecutive_odds(arr);
    println!("Result: {}", result);
}
