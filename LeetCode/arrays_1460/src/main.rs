struct Solution;

impl Solution {
    pub fn can_be_equal(target: Vec<i32>, arr: Vec<i32>) -> bool {
        if target.len() != arr.len() {
            return false;
        }

        let mut target_sorted = target.clone();
        let mut arr_sorted = arr.clone();

        target_sorted.sort_unstable();
        arr_sorted.sort_unstable();

        target_sorted == arr_sorted
    }
}

fn main() {
    println!("Hello, world!");

    let target = vec![1, 2, 3, 4];
    let arr = vec![2, 4, 1, 3];
    let result = Solution::can_be_equal(target, arr);
    println!("Result: {}", result);
}
