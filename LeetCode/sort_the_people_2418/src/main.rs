struct Solution;

impl Solution {
    pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut people: Vec<(i32, String)> = heights.into_iter().zip(names.into_iter()).collect();
        // Sort by height in descending order
        people.sort_by(|a, b| b.0.cmp(&a.0));
        // Extract names into a new Vec<String>
        people.into_iter().map(|(_, name)| name).collect()
    }
}

fn main() {
    println!("Hello, world!");

    let names = vec!["Alex".to_string(), "Kevin".to_string(), "Henry".to_string(), "John".to_string(), "George".to_string()];
    let heights = vec![5, 6, 7, 8, 9];
    let result = Solution::sort_people(names, heights);
    println!("{:?}", result);
}
