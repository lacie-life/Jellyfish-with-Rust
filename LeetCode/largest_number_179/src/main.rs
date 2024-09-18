struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str: Vec<String> = nums.into_iter().map(|num| num.to_string()).collect();

        nums_str.sort_unstable_by(|a, b| {
            let order1 = format!("{}{}", a, b);
            let order2 = format!("{}{}", b, a);
            order2.cmp(&order1)
        });

        if nums_str[0] == "0" {
            return "0".to_string();
        }

        nums_str.join("")
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![3, 30, 34, 5, 9];
    let result = Solution::largest_number(nums);
    println!("Result: {}", result);
}
