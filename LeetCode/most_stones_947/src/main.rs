use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn remove_stones(stones: Vec<Vec<i32>>) -> i32 {
        let mut graph = vec![vec![]; stones.len()];
        for i in 0..stones.len() {
            for j in i + 1..stones.len() {
                if stones[i][0] == stones[j][0] || stones[i][1] == stones[j][1] {
                    graph[i].push(j);
                    graph[j].push(i);
                }
            }
        }

        let mut visited = vec![false; stones.len()];
        let mut components = 0;

        for i in 0..stones.len() {
            if !visited[i] {
                components += 1;
                let mut stack = VecDeque::new();
                stack.push_back(i);
                while let Some(node) = stack.pop_back() {
                    if !visited[node] {
                        visited[node] = true;
                        for &neighbor in &graph[node] {
                            if !visited[neighbor] {
                                stack.push_back(neighbor);
                            }
                        }
                    }
                }
            }
        }

        (stones.len() as i32) - components
    }
}


fn main() {
    println!("Hello, world!");

    let stones = vec![vec![0, 0], vec![0, 1], vec![1, 0], vec![1, 2], vec![2, 1], vec![2, 2]];
    let result = Solution::remove_stones(stones);
    println!("result = {}", result);
}
