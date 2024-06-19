struct Solution(Vec<i32>, i32, i32);

impl Solution {
    pub fn min_days(bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        let mut left = 1;
        let mut right = *bloom_day.iter().max().unwrap();
        let mut ans = -1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::check(&bloom_day, m, k, mid) {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans
    }

    fn check(bloom_day: &Vec<i32>, m: i32, k: i32, mid: i32) -> bool {
        let mut flowers = 0;
        let mut bouquets = 0;
        for &day in bloom_day.iter() {
            if day <= mid {
                flowers += 1;
                if flowers == k {
                    bouquets += 1;
                    flowers = 0;
                }
            } else {
                flowers = 0;
            }
        }
        bouquets >= m
    }
}

fn main() {
    println!("Hello, world!");

    let bloom_day = vec![1, 10, 3, 10, 2];
    let m = 3;
    let k = 1;
    let result = Solution::min_days(bloom_day, m, k);

    println!("Result: {}", result);
}
