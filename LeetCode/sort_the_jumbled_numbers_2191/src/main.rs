struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut nums_mapped = nums.iter().enumerate().map(|(index, &num)| {
            // Convert each number to its mapped value
            let mapped_num = num.to_string().chars().map(|c| {
                let digit = c.to_digit(10).unwrap() as usize;
                mapping[digit].to_string()
            }).collect::<String>().parse::<i32>().unwrap();

            (index, num, mapped_num)
        }).collect::<Vec<_>>();

        // Sort based on the mapped values, maintaining original order for duplicates
        nums_mapped.sort_by(|a, b| a.2.cmp(&b.2).then_with(|| a.0.cmp(&b.0)));

        // Extract the original numbers in their new sorted order
        nums_mapped.into_iter().map(|(_, num, _)| num).collect()
    }
}

fn main() {
    println!("Hello, world!");

    let mapping = vec![8, 9, 4, 0, 2, 1, 3, 5, 7, 6];
    let nums = vec![991, 338, 38];
    let result = Solution::sort_jumbled(mapping, nums);
    println!("{:?}", result);

}
