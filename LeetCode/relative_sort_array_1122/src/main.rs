struct Solution;

impl Solution {
    pub fn relative_sort_array(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
        let mut count = vec![0; 1001];
        let mut result = Vec::new();

        for &num in &arr1 {
            count[num as usize] += 1;
        }

        for &num in &arr2 {
            while count[num as usize] > 0 {
                result.push(num);
                count[num as usize] -= 1;
            }
        }

        for num in 0..1001 {
            while count[num] > 0 {
                result.push(num as i32);
                count[num] -= 1;
            }
        }

        result
    }
}

fn main() {
    println!("Hello, world!");
    let arr1 = vec![2, 3, 1, 3, 2, 4, 6, 7, 9, 2, 19];
    let arr2 = vec![2, 1, 4, 3, 9, 6];
    let result = Solution::relative_sort_array(arr1, arr2);
    println!("{:?}", result);
}

