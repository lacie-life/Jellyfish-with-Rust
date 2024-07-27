use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn minimum_cost(source: String, target: String, original: Vec<char>, changed: Vec<char>, cost: Vec<i32>) -> i64 {
        let mut transformation_cost: HashMap<(char, char), i32> = HashMap::new();
        let mut min_cost: HashMap<(char, char), i32> = HashMap::new();

        // Populate the HashMap with the minimum cost for each transformation
        for i in 0..original.len() {
            let key = (original[i], changed[i]);
            let entry = transformation_cost.entry(key).or_insert(cost[i]);
            if cost[i] < *entry {
                *entry = cost[i];
            }
        }

        // Initialize min_cost with direct transformations
        for (&(from, to), &c) in &transformation_cost {
            min_cost.insert((from, to), c);
        }

        // Floyd-Warshall algorithm to find the minimum cost for all possible transformations
        for k in "abcdefghijklmnopqrstuvwxyz".chars() {
            for i in "abcdefghijklmnopqrstuvwxyz".chars() {
                for j in "abcdefghijklmnopqrstuvwxyz".chars() {
                    if let (Some(&ik), Some(&kj)) = (min_cost.get(&(i, k)), min_cost.get(&(k, j))) {
                        let ij = min_cost.entry((i, j)).or_insert(i32::MAX);
                        if ik + kj < *ij {
                            *ij = ik + kj;
                        }
                    }
                }
            }
        }

        let mut total_cost: i64 = 0;

        // Iterate through each character in source and target
        for (s_char, t_char) in source.chars().zip(target.chars()) {
            if s_char != t_char {
                if let Some(&trans_cost) = min_cost.get(&(s_char, t_char)) {
                    total_cost += trans_cost as i64;
                } else {
                    return -1; // Transformation not possible
                }
            }
        }

        total_cost
    }
}

fn main() {
    println!("Hello, world!");

    let source = "abcd".to_string();
    let target = "acbe".to_string();
    let original = vec!['a', 'b', 'c', 'c' , 'e' ,'d'];
    let changed = vec!['b','c','b','e','b','e'];
    let cost = vec![2, 5, 5, 1, 2, 20];

    let result = Solution::minimum_cost(source, target, original, changed, cost);
    println!("Result: {}", result);
}
