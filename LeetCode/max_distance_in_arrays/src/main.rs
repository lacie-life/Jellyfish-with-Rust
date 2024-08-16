struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut min_val = arrays[0][0];
        let mut max_val = arrays[0][arrays[0].len() - 1];
        let mut max_distance = 0;

        for array in arrays.iter().skip(1) {
            max_distance = max_distance.max((array[array.len() - 1] - min_val).abs());
            max_distance = max_distance.max((max_val - array[0]).abs());

            min_val = min_val.min(array[0]);
            max_val = max_val.max(array[array.len() - 1]);
        }

        max_distance
    }
}

fn main() {
    println!("Hello, world!");

    let arrays = vec![vec![1, 2, 3], vec![4, 5], vec![1, 2, 3]];
    let result = Solution::max_distance(arrays);
    println!("Result: {}", result);
}
