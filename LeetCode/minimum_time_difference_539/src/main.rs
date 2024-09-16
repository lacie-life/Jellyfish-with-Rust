struct Solution;

impl Solution {
    pub fn find_min_difference(time_points: Vec<String>) -> i32 {
        // Step 1: Parse the time points into minutes since midnight
        let mut minutes: Vec<i32> = time_points.iter().map(|time| {
            let parts: Vec<i32> = time.split(':')
                .map(|part| part.parse().unwrap())
                .collect();
            parts[0] * 60 + parts[1]
        }).collect();

        // Step 2: Sort the list of minutes
        minutes.sort_unstable();

        // Step 3: Calculate the differences between consecutive time points
        let mut min_diff = i32::MAX;
        for i in 1..minutes.len() {
            min_diff = min_diff.min(minutes[i] - minutes[i - 1]);
        }

        // Step 4: Consider the circular difference between the last and first time points
        let circular_diff = 1440 + minutes[0] - minutes[minutes.len() - 1];
        min_diff = min_diff.min(circular_diff);

        // Step 5: Return the minimum difference
        min_diff
    }
}

fn main() {
    println!("Hello, world!");

    let time_points = vec!["23:59".to_string(), "00:00".to_string()];
    let min_diff = Solution::find_min_difference(time_points);
    println!("Minimum difference: {}", min_diff);
}
