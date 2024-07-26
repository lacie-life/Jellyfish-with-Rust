struct Solution;

impl Solution {
    pub fn find_the_city(n: i32, edges: Vec<Vec<i32>>, distance_threshold: i32) -> i32 {
        let n = n as usize;
        let mut dist = vec![vec![i32::MAX; n]; n];

        for i in 0..n {
            dist[i][i] = 0;
        }

        for edge in edges {
            let (u, v, w) = (edge[0] as usize, edge[1] as usize, edge[2]);
            dist[u][v] = w;
            dist[v][u] = w;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dist[i][k] != i32::MAX && dist[k][j] != i32::MAX {
                        dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                    }
                }
            }
        }

        let mut min_reachable = n;
        let mut result_city = 0;

        for i in 0..n {
            let reachable = dist[i].iter().filter(|&&d| d <= distance_threshold).count();
            if reachable <= min_reachable {
                min_reachable = reachable;
                result_city = i;
            }
        }

        result_city as i32
    }
}

fn main() {
    println!("Hello, world!");

    let n = 4;
    let edges = vec![vec![0, 1, 3], vec![1, 2, 1], vec![1, 3, 4], vec![2, 3, 1]];
    let distance_threshold = 4;
    let result = Solution::find_the_city(n, edges, distance_threshold);
    println!("result = {}", result);

    let n = 5;
    let edges = vec![vec![0, 1, 2], vec![0, 4, 8], vec![1, 2, 3], vec![1, 4, 2], vec![2, 3, 1], vec![3, 4, 1]];
    let distance_threshold = 2;
    let result = Solution::find_the_city(n, edges, distance_threshold);
    println!("result = {}", result);
}
