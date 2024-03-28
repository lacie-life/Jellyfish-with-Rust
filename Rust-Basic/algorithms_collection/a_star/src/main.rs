use std::collections::{BinaryHeap, HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Node {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Cost {
    g: usize,
    h: usize,
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    node: Node,
    cost: Cost,
    parent: Option<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.cost.g + self.cost.h).cmp(&(other.cost.g + other.cost.h)).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: Node, b: Node) -> usize {
    // Example: Manhattan distance
    ((a.x as isize - b.x as isize).abs() + (a.y as isize - b.y as isize).abs()) as usize
}

fn reconstruct_path(came_from: HashMap<Node, Node>, mut current: Node) -> Vec<Node> {
    let mut path = vec![current];
    while let Some(&parent) = came_from.get(&current) {
        path.push(parent);
        current = parent;
    }
    path.reverse();
    path
}

fn astar(start: Node, goal: Node, grid: Vec<Vec<usize>>) -> Option<Vec<Node>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();

    g_score.insert(start, 0);

    open_set.push(State {
        node: start,
        cost: Cost { g: 0, h: heuristic(start, goal) },
        parent: None,
    });

    while let Some(State { node, cost, .. }) = open_set.pop() {
        if node == goal {
            return Some(reconstruct_path(came_from, node));
        }

        for neighbor in vec![
            Node { x: node.x + 1, y: node.y },
            Node { x: node.x, y: node.y + 1 },
            Node { x: node.x.checked_sub(1)?, y: node.y },
            Node { x: node.x, y: node.y.checked_sub(1)? },
        ] {
            if neighbor.x >= grid.len() || neighbor.y >= grid[0].len() || grid[neighbor.x][neighbor.y] == 1 {
                continue;
            }

            let tentative_g_score = g_score.get(&node).unwrap_or(&usize::MAX) + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                came_from.insert(neighbor, node);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(State {
                    node: neighbor,
                    cost: Cost { g: tentative_g_score, h: heuristic(neighbor, goal) },
                    parent: Some(node),
                });
            }
        }
    }

    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![1, 1, 1, 1, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 1],
        vec![0, 0, 0, 0, 0],
    ];

    let start = Node { x: 0, y: 0 };
    let goal = Node { x: 4, y: 4 };

    if let Some(path) = astar(start, goal, grid) {
        println!("Path found: {:?}", path);
    } else {
        println!("No path found.");
    }
}
