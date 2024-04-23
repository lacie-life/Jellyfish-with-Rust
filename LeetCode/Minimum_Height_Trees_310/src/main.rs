use std::collections::{HashMap, VecDeque};
struct Solution(i32, Vec<Vec<i32>>);

impl Solution {
    pub fn find_min_height_trees(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        // Base case: if there's only one node, it's the root of a minHeightTree
        if n == 1 {
            return vec![0];
        }

        // Each node will be an index in an adjacency list
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();
        // The degree count for each node
        let mut degrees: HashMap<i32, i32> = HashMap::new();

        // Construct the graph
        for edge in edges.iter() {
            let node_a = edge[0];
            let node_b = edge[1];
            adjacency_list.entry(node_a).or_insert(Vec::new()).push(node_b);
            adjacency_list.entry(node_b).or_insert(Vec::new()).push(node_a);
            *degrees.entry(node_a).or_insert(0) += 1;
            *degrees.entry(node_b).or_insert(0) += 1;
        }

        // Initialize a queue for processing leaf nodes (nodes with degree 1)
        let mut processing_queue: VecDeque<i32> = VecDeque::new();
        for i in 0..n {
            if *degrees.get(&i).unwrap_or(&0) == 1 {
                processing_queue.push_back(i);
            }
        }

        // Vector to hold the minimum height tree roots
        let mut min_height_roots: Vec<i32> = Vec::new();

        // Process the graph
        while !processing_queue.is_empty() {
            // Start a new level
            min_height_roots.clear();
            let level_size = processing_queue.len(); // Number of nodes in the current level

            // Process all nodes in the current level
            for _ in 0..level_size {
                if let Some(current_node) = processing_queue.pop_front() {
                    // Add the current node to this level's results
                    min_height_roots.push(current_node);

                    // Decrease the degree of adjacent nodes and enqueue new leaf nodes
                    if let Some(adjacent_nodes) = adjacency_list.get(&current_node) {
                        for &adjacent_node in adjacent_nodes {
                            if let Some(degree) = degrees.get_mut(&adjacent_node) {
                                *degree -= 1;
                                if *degree == 1 {
                                    processing_queue.push_back(adjacent_node);
                                }
                            }
                        }
                    }
                }
            }
        }

        // min_height_roots now contains the roots of trees that have the minimum height
        min_height_roots
    }
}


fn main() {
    println!("Hello, world!");

    let n = 6;
    let edges = vec![vec![3, 0], vec![3, 1], vec![3, 2], vec![3, 4], vec![5, 4]];
    let res = Solution::find_min_height_trees(n, edges);
    println!("{:?}", res);

    let n = 4;
    let edges = vec![vec![1, 0], vec![1, 2], vec![1, 3]];
    let res = Solution::find_min_height_trees(n, edges);
    println!("{:?}", res);
}
