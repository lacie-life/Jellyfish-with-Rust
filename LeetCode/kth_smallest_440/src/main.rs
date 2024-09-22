struct Solution;

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        let mut ans = 1;
        let mut i = 1;

        while i < k {
            let gap = Self::get_gap(ans, ans + 1, n as i64);
            if i + gap <= k {
                i += gap;
                ans += 1;
            } else {
                i += 1;
                ans *= 10;
            }
        }

        ans.try_into().unwrap()
    }

    fn get_gap(mut a: i64, mut b: i64, n: i64) -> i32 {
        let mut gap = 0;
        while a <= n {
            gap += std::cmp::min(n + 1, b) - a;
            a *= 10;
            b *= 10;
        }
        gap as i32
    }
}

fn main() {
    println!("Hello, world!");

    let n = 13;
    let k = 2;
    let result = Solution::find_kth_number(n, k);
    println!("Result: {}", result);
}
