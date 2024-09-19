use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut mem = HashMap::new();
        Self::ways(&expression, &mut mem)
    }

    fn ways(s: &str, mem: &mut HashMap<String, Vec<i32>>) -> Vec<i32> {
        if let Some(res) = mem.get(s) {
            return res.clone();
        }

        let mut ans = Vec::new();

        for (i, c) in s.chars().enumerate() {
            if c == '+' || c == '-' || c == '*' {
                let left = Self::ways(&s[0..i], mem);
                let right = Self::ways(&s[i+1..], mem);

                for &a in &left {
                    for &b in &right {
                        match c {
                            '+' => ans.push(a + b),
                            '-' => ans.push(a - b),
                            '*' => ans.push(a * b),
                            _ => unreachable!(),
                        }
                    }
                }
            }
        }

        if ans.is_empty() {
            ans.push(s.parse::<i32>().unwrap());
        }

        mem.insert(s.to_string(), ans.clone());
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let expression = "2*3-4*5".to_string();
    let result = Solution::diff_ways_to_compute(expression);
    println!("{:?}", result);
}
