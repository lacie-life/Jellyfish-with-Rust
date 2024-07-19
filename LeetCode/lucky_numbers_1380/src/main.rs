use std::cmp;

struct Solution;

impl Solution {
    pub fn lucky_numbers(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        for row in &matrix {
            let min_index = row.iter().enumerate().min_by_key(|&(_, &val)| val).unwrap().0;
            if row[min_index] == Self::max_num_of_column(&matrix, min_index) {
                return vec![row[min_index]];
            }
        }
        vec![]
    }

    fn max_num_of_column(matrix: &Vec<Vec<i32>>, col: usize) -> i32 {
        let mut res = i32::MIN;
        for row in matrix {
            res = cmp::max(res, row[col]);
        }
        res
    }
}

fn main() {
    println!("Hello, world!");

    let matrix = vec![vec![3, 7, 8], vec![9, 11, 13], vec![15, 16, 17]];
    let lucky_numbers = Solution::lucky_numbers(matrix);
    println!("{:?}", lucky_numbers);
}
