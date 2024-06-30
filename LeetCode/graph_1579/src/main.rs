struct UnionFind {
    count: i32,
    id: Vec<i32>,
    rank: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let count = n as i32;
        let id = (0..n as i32).collect();
        let rank = vec![0; n];
        UnionFind { count, id, rank }
    }

    fn find(&mut self, u: i32) -> i32 {
        if self.id[u as usize] != u {
            self.id[u as usize] = self.find(self.id[u as usize]);
        }
        self.id[u as usize]
    }

    fn union_by_rank(&mut self, u: i32, v: i32) -> bool {
        let i = self.find(u);
        let j = self.find(v);
        if i == j {
            return false;
        }
        if self.rank[i as usize] < self.rank[j as usize] {
            self.id[i as usize] = j;
        } else if self.rank[i as usize] > self.rank[j as usize] {
            self.id[j as usize] = i;
        } else {
            self.id[i as usize] = j;
            self.rank[j as usize] += 1;
        }
        self.count -= 1;
        true
    }

    fn get_count(&self) -> i32 {
        self.count
    }
}

struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let mut alice = UnionFind::new(n as usize);
        let mut bob = UnionFind::new(n as usize);
        let mut required_edges = 0;

        let mut edges = edges.clone();
        edges.sort_by(|a, b| b[0].cmp(&a[0]));

        for edge in &edges {
            let edge_type = edge[0];
            let u = edge[1] - 1;
            let v = edge[2] - 1;
            match edge_type {
                3 => {
                    if alice.union_by_rank(u, v) | bob.union_by_rank(u, v) {
                        required_edges += 1;
                    }
                }
                2 => {
                    if bob.union_by_rank(u, v) {
                        required_edges += 1;
                    }
                }
                1 => {
                    if alice.union_by_rank(u, v) {
                        required_edges += 1;
                    }
                }
                _ => (),
            }
        }

        if alice.get_count() == 1 && bob.get_count() == 1 {
            (edges.len() as i32) - required_edges
        } else {
            -1
        }
    }
}

fn main() {
    let n = 4;
    let edges = vec![
        vec![3, 1, 2],
        vec![3, 2, 3],
        vec![1, 1, 3],
        vec![1, 2, 4],
        vec![1, 1, 2],
        vec![2, 3, 4],
    ];
    let result = Solution::max_num_edges_to_remove(n, edges);
    println!("{}", result);
}

