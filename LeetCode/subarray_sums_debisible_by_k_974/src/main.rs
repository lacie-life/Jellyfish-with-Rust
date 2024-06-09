use std::collections::HashMap;
struct Solution(Vec<i32>, i32);

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let mut map: HashMap<i32, i32> = HashMap::new();
        map.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;
        for num in nums {
            sum += num;
            let key = ((sum % k) + k) % k;
            count += *map.get(&key).unwrap_or(&0);
            *map.entry(key).or_insert(0) += 1;
        }
        count
    }
}


fn main() {
    println!("Hello, world!");

    let nums = vec![4,5,0,-2,-3,1];
    let k = 5;
    let result = Solution::subarrays_div_by_k(nums, k);
    println!("Result: {}", result);
}
