use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn build_matrix(k: i32, row_conditions: Vec<Vec<i32>>, col_conditions: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let row_order = Self::topological_sort(k, &row_conditions);
        let col_order = Self::topological_sort(k, &col_conditions);

        if row_order.is_empty() || col_order.is_empty() {
            return vec![];
        }

        let mut matrix = vec![vec![0; k as usize]; k as usize];
        let mut position = HashMap::new();

        for (i, &num) in row_order.iter().enumerate() {
            position.entry(num).or_insert((0, 0)).0 = i;
        }

        for (i, &num) in col_order.iter().enumerate() {
            position.entry(num).or_insert((0, 0)).1 = i;
        }

        for (&num, &(r, c)) in &position {
            matrix[r][c] = num;
        }

        matrix
    }

    fn topological_sort(k: i32, conditions: &[Vec<i32>]) -> Vec<i32> {
        let mut graph = vec![vec![]; k as usize + 1];
        let mut in_degree = vec![0; k as usize + 1];

        for condition in conditions {
            graph[condition[0] as usize].push(condition[1]);
            in_degree[condition[1] as usize] += 1;
        }

        let mut queue = VecDeque::new();
        for i in 1..=k as usize {
            if in_degree[i] == 0 {
                queue.push_back(i as i32);
            }
        }

        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &next in &graph[node as usize] {
                in_degree[next as usize] -= 1;
                if in_degree[next as usize] == 0 {
                    queue.push_back(next);
                }
            }
        }

        if order.len() as i32 == k {
            order
        } else {
            vec![]
        }
    }
}

fn main() {
    println!("Hello, world!");

    let k = 3;
    let row_conditions = vec![vec![1, 2], vec![3, 2]];
    let col_conditions = vec![vec![2, 1], vec![3, 2]];
    let result = Solution::build_matrix(k, row_conditions, col_conditions);
    println!("{:?}", result);
}
