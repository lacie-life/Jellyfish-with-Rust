struct Solution(String, i32, i32);

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        let (mut count_ab, mut count_ba) = (0, 0);
        let mut chars = s.chars().collect::<Vec<char>>();
        let mut stack = Vec::new();

        // First pass: Optimize for the higher value operation
        if x > y {
            for c in chars {
                if c == 'b' && !stack.is_empty() && *stack.last().unwrap() == 'a' {
                    stack.pop(); // Remove 'a'
                    count_ab += 1; // Increment count for "ab"
                } else {
                    stack.push(c);
                }
            }
        } else {
            for c in chars {
                if c == 'a' && !stack.is_empty() && *stack.last().unwrap() == 'b' {
                    stack.pop(); // Remove 'b'
                    count_ba += 1; // Increment count for "ba"
                } else {
                    stack.push(c);
                }
            }
        }

        // Second pass: Optimize for the lower value operation
        let mut second_pass_stack = Vec::new();
        if x <= y {
            for c in stack {
                if c == 'b' && !second_pass_stack.is_empty() && *second_pass_stack.last().unwrap() == 'a' {
                    second_pass_stack.pop(); // Remove 'a'
                    count_ab += 1; // Increment count for "ab"
                } else {
                    second_pass_stack.push(c);
                }
            }
        } else {
            for c in stack {
                if c == 'a' && !second_pass_stack.is_empty() && *second_pass_stack.last().unwrap() == 'b' {
                    second_pass_stack.pop(); // Remove 'b'
                    count_ba += 1; // Increment count for "ba"
                } else {
                    second_pass_stack.push(c);
                }
            }
        }

        // Calculate total points
        (count_ab * x) + (count_ba * y)
    }
}

fn main() {
    println!("Hello, world!");

    let s = "cdbcbbaaabab".to_string();
    let x = 4;
    let y = 5;
    let result = Solution::maximum_gain(s, x, y);

    println!("Result: {}", result);

    let s = "aabbaaxybbaabb".to_string();
    let x = 5;
    let y = 4;
    let result = Solution::maximum_gain(s, x, y);

    println!("Result: {}", result);
}
