struct Solution(Vec<i32>);

impl Solution {
    pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::backtrack(&mut result, &mut Vec::new(), 0, &nums);
        result
    }

    fn backtrack(result: &mut Vec<Vec<i32>>, current: &mut Vec<i32>, start: usize, nums: &Vec<i32>) {
        result.push(current.clone());

        for i in start..nums.len() {
            current.push(nums[i]);
            Self::backtrack(result, current, i + 1, nums);
            current.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![1, 2, 3];
    let result = Solution::subsets(nums);
    for subset in result {
        println!("{:?}", subset);
    }
}
