use std::collections::{HashSet, VecDeque};

struct Solution(Vec<String>, String);

impl Solution {
    pub fn open_lock(deadends: Vec<String>, target: String) -> i32 {
        let start = "0000";

        // Deadend set => constant time lookup
        let mut deadend_set = HashSet::new();
        for deadend in deadends {
            deadend_set.insert(deadend);
        }

        // Output
        let mut queue = VecDeque::new();
        queue.push_back((start.to_string(), 0));

        // BFS
        let mut visited = HashSet::new();
        visited.insert(start.to_string());

        if deadend_set.contains(start) {
            return -1;
        }

        while let Some((combination, turns)) = queue.pop_front() {
            if combination == target {
                return turns;
            }

            for i in 0..4 {
                let up = Solution::turn_up(combination.chars().nth(i).unwrap());
                let down = Solution::turn_down(combination.chars().nth(i).unwrap());

                let mut up_combination = combination.clone();
                up_combination.replace_range(i..i + 1, &up.to_string());

                let mut down_combination = combination.clone();
                down_combination.replace_range(i..i + 1, &down.to_string());

                if !visited.contains(&up_combination) && !deadend_set.contains(&up_combination) {
                    visited.insert(up_combination.clone());
                    queue.push_back((up_combination, turns + 1));
                }

                if !visited.contains(&down_combination) && !deadend_set.contains(&down_combination) {
                    visited.insert(down_combination.clone());
                    queue.push_back((down_combination, turns + 1));
                }
            }
        }

        -1
    }

    fn turn_up(curr: char) -> char {
        if curr == '9' {
            return '0';
        }
        return (curr as u8 + 1) as char;
    }

    fn turn_down(curr: char) -> char {
        if curr == '0' {
            return '9';
        }
        return (curr as u8 - 1) as char;
    }
}


fn main() {
    println!("Hello, world!");

    let deadends = vec!["0201", "0101", "0102", "1212", "2002"];
    let target = "0202";

    let result = Solution::open_lock(deadends.iter().map(|s| s.to_string()).collect(), target.to_string());
    println!("Result: {}", result);

    let deadends = vec!["8888"];
    let target = "0009";
    let result = Solution::open_lock(deadends.iter().map(|s| s.to_string()).collect(), target.to_string());
    println!("Result: {}", result);

    let deadends = vec!["8887", "8889", "8878", "8898", "8788", "8988", "7888", "9888"];
    let target = "8888";
    let result = Solution::open_lock(deadends.iter().map(|s| s.to_string()).collect(), target.to_string());
    println!("Result: {}", result);
}
