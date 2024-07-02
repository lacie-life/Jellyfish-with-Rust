use std::collections::HashMap;

struct Solution(Vec<i32>, Vec<i32>);

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut map = HashMap::new();
        for num in nums1 {
            let count = map.entry(num).or_insert(0);
            *count += 1;
        }

        let mut res = Vec::new();
        for num in nums2 {
            if let Some(count) = map.get_mut(&num) {
                if *count > 0 {
                    res.push(num);
                    *count -= 1;
                }
            }
        }

        res
    }
}

fn main() {
    println!("Hello, world!");

    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    let result = Solution::intersect(nums1, nums2);
    println!("{:?}", result);

    let nums1 = vec![4, 9, 5];
    let nums2 = vec![9, 4, 9, 8, 4];
    let result = Solution::intersect(nums1, nums2);
    println!("{:?}", result);
}
