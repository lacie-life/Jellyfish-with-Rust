struct Solution;

impl Solution {
    pub fn count_seniors(details: Vec<String>) -> i32 {
        let mut count = 0;
        for detail in details {
            if let Ok(age) = detail[11..13].parse::<u8>() {
                if age > 60 {
                    count += 1;
                }
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");

    let details = vec![
        "1234567890M65A1".to_string(),
        "0987654321F45B2".to_string(),
        "1122334455M70C3".to_string(),
    ];
    let result = Solution::count_seniors(details);
    println!("Number of senior citizens: {}", result);
}
