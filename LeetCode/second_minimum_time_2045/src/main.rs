use std::collections::VecDeque;
use std::cmp;

struct Solution;

impl Solution {
    pub fn second_minimum(n: i32, edges: Vec<Vec<i32>>, time: i32, change: i32) -> i32 {
        let n = n as usize;
        let time = time as i32;
        let change = change as i32;

        let mut graph = vec![Vec::new(); n + 1];
        let mut q = VecDeque::new();
        q.push_back((1, 0));

        let mut min_time = vec![vec![i32::MAX; 2]; n + 1];
        min_time[1][0] = 0;

        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }

        while let Some((i, prev_time)) = q.pop_front() {
            let num_change_signal = prev_time / change;
            let wait_time = if num_change_signal % 2 == 0 {
                0
            } else {
                change - prev_time % change
            };
            let new_time = prev_time + wait_time + time;
            for &j in &graph[i] {
                if new_time < min_time[j][0] {
                    min_time[j][0] = new_time;
                    q.push_back((j, new_time));
                } else if min_time[j][0] < new_time && new_time < min_time[j][1] {
                    if j == n {
                        return new_time;
                    }
                    min_time[j][1] = new_time;
                    q.push_back((j, new_time));
                }
            }
        }

        panic!("No solution found");
    }
}


fn main() {
    println!("Hello, world!");

    let n = 5;
    let edges = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![3, 4], vec![4, 5]];
    let time = 3;
    let change = 5;
    let result = Solution::second_minimum(n, edges, time, change);
    println!("result = {}", result);
}
