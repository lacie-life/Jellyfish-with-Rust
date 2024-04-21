struct Solution(i32, Vec<Vec<i32>>, i32, i32);

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, start: i32, end: i32) -> bool {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut visited = vec![false; n as usize];
        Self::dfs(&graph, &mut visited, start, end)
    }

    fn dfs(graph: &Vec<Vec<i32>>, visited: &mut Vec<bool>, start: i32, end: i32) -> bool {
        if start == end {
            return true;
        }
        visited[start as usize] = true;
        for &next in &graph[start as usize] {
            if !visited[next as usize] && Self::dfs(graph, visited, next, end) {
                return true;
            }
        }
        false
    }
}


fn main() {
    println!("Hello, world!");

    let n = 3;
    let edges = vec![vec![0,1],vec![1,2],vec![2,0]];
    let start = 0;
    let end = 2;

    let result = Solution::valid_path(n, edges, start, end);
    println!("result = {}", result);
}
