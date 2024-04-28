struct Solution(i32, Vec<Vec<i32>>);

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut graph = vec![vec![]; n as usize];
        for edge in edges {
            graph[edge[0] as usize].push(edge[1]);
            graph[edge[1] as usize].push(edge[0]);
        }
        let mut count = vec![1; n as usize];
        let mut res = vec![0; n as usize];
        Self::dfs1(0, -1, &graph, &mut count, &mut res);
        Self::dfs2(0, -1, &graph, &count, &mut res);
        res
    }

    fn dfs1(node: usize, parent: i32, graph: &Vec<Vec<i32>>, count: &mut Vec<i32>, res: &mut Vec<i32>) {
        for &child in &graph[node] {
            if child as i32 == parent {
                continue;
            }
            Self::dfs1(child as usize, node as i32, graph, count, res);
            count[node] += count[child as usize];
            res[node] += res[child as usize] + count[child as usize];
        }
    }

    fn dfs2(node: usize, parent: i32, graph: &Vec<Vec<i32>>, count: &Vec<i32>, res: &mut Vec<i32>) {
        for &child in &graph[node] {
            if child as i32 == parent {
                continue;
            }
            res[child as usize] = res[node] - count[child as usize] + count.len() as i32 - count[child as usize];
            Self::dfs2(child as usize, node as i32, graph, count, res);
        }
    }
}

fn main() {
    println!("Hello, world!");

    let n = 6;
    let edges = vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![2, 4], vec![2, 5]];
    let res = Solution::sum_of_distances_in_tree(n, edges);
    println!("{:?}", res);

    let n = 1;
    let edges = vec![];
    let res = Solution::sum_of_distances_in_tree(n, edges);
    println!("{:?}", res);

    let n = 2;
    let edges = vec![vec![1, 0]];
    let res = Solution::sum_of_distances_in_tree(n, edges);
    println!("{:?}", res);
}
