struct Solution;

impl Solution {
    pub fn combination_sum2(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut candidates = candidates;
        candidates.sort();
        let mut result = Vec::new();
        let mut combination = Vec::new();
        Self::backtrack(&candidates, target, 0, &mut combination, &mut result);
        result
    }

    fn backtrack(candidates: &[i32], target: i32, start: usize, combination: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {
        if target == 0 {
            result.push(combination.clone());
            return;
        }
        for i in start..candidates.len() {
            if i > start && candidates[i] == candidates[i - 1] {
                continue; // skip duplicates
            }
            if candidates[i] > target {
                break;
            }
            combination.push(candidates[i]);
            Self::backtrack(candidates, target - candidates[i], i + 1, combination, result);
            combination.pop();
        }
    }
}

fn main() {
    println!("Hello, world!");

    let candidates = vec![10,1,2,7,6,1,5];
    let target = 8;
    let result = Solution::combination_sum2(candidates, target);
    println!("{:?}", result);

    let candidates = vec![2,5,2,1,2];
    let target = 5;
    let result = Solution::combination_sum2(candidates, target);
    println!("{:?}", result);
}
