struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut ans = Vec::with_capacity(n as usize);
        let mut curr = 1;

        while ans.len() < n as usize {
            ans.push(curr);
            if curr * 10 <= n {
                curr *= 10;
            } else {
                while curr % 10 == 9 || curr == n {
                    curr /= 10;
                }
                curr += 1;
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");

    let n = 13;
    let result = Solution::lexical_order(n);
    println!("{:?}", result);
}
