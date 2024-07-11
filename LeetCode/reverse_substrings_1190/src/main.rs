struct Solution(String);

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let mut stack: Vec<String> = vec![String::new()];
        for c in s.chars() {
            match c {
                '(' => stack.push(String::new()),
                ')' => {
                    let mut temp = stack.pop().unwrap();
                    temp = temp.chars().rev().collect();
                    if let Some(last) = stack.last_mut() {
                        last.push_str(&temp);
                    }
                },
                _ => {
                    if let Some(last) = stack.last_mut() {
                        last.push(c);
                    }
                },
            }
        }
        stack.pop().unwrap_or_default()
    }
}

fn main() {
    println!("Hello, world!");

    let s = String::from("(u(love)i)");
    let result = Solution::reverse_parentheses(s);
    println!("result: {}", result);

    let s = String::from("(ed(et(oc))el)");
    let result = Solution::reverse_parentheses(s);
    println!("result: {}", result);
}
