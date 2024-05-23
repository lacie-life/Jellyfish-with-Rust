
struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;
        let mut cnt = vec![0; 1010];
        let n = nums.len();

        fn dfs(nums: &Vec<i32>, k: i32, cnt: &mut Vec<i32>, i: usize, ans: &mut i32, n: usize) {
            if i >= n {
                *ans += 1;
                return;
            }
            dfs(nums, k, cnt, i + 1, ans, n);
            let ok1 = nums[i] + k >= 1010 || cnt[(nums[i] + k) as usize] == 0;
            let ok2 = nums[i] - k < 0 || cnt[(nums[i] - k) as usize] == 0;
            if ok1 && ok2 {
                cnt[nums[i] as usize] += 1;
                dfs(nums, k, cnt, i + 1, ans, n);
                cnt[nums[i] as usize] -= 1;
            }
        }

        dfs(&nums, k, &mut cnt, 0, &mut ans, n);
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![2, 4, 6];
    let k = 2;
    let result = Solution::beautiful_subsets(nums, k);
    println!("Result: {}", result);
}
