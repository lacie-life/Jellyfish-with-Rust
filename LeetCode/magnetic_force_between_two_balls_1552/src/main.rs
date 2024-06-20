struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn max_distance(position: Vec<i32>, m: i32) -> i32 {
        let mut position = position;
        position.sort();
        let mut left = 1;
        let mut right = position.last().unwrap() - position.first().unwrap();
        let mut ans = 0;
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::check(&position, mid, m) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        ans
    }

    fn check(position: &Vec<i32>, mid: i32, m: i32) -> bool {
        let mut count = 1;
        let mut last = position[0];
        for &pos in position.iter().skip(1) {
            if pos - last >= mid {
                last = pos;
                count += 1;
            }
        }
        count >= m
    }
}


fn main() {
    println!("Hello, world!");

    let position = vec![1, 2, 3, 4, 7];
    let m = 3;
    let result = Solution::max_distance(position, m);
    println!("result = {}", result);
}
