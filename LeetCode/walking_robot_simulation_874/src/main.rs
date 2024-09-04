use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
        let mut x = 0;
        let mut y = 0;
        let mut direction = 0;
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut obstacle_set = HashSet::new();

        for obstacle in obstacles {
            obstacle_set.insert((obstacle[0], obstacle[1]));
        }

        let mut max_distance_sq = 0;

        for command in commands {
            match command {
                -2 => direction = (direction + 3) % 4, // Turn left
                -1 => direction = (direction + 1) % 4, // Turn right
                1..=9 => {
                    for _ in 0..command {
                        let next_x = x + directions[direction].0;
                        let next_y = y + directions[direction].1;
                        if !obstacle_set.contains(&(next_x, next_y)) {
                            x = next_x;
                            y = next_y;
                            max_distance_sq = max_distance_sq.max(x * x + y * y);
                        } else {
                            break;
                        }
                    }
                },
                _ => (),
            }
        }

        max_distance_sq
    }
}

fn main() {
    println!("Hello, world!");

    let commands = vec![4, -1, 3];
    let obstacles = vec![];
    let result = Solution::robot_sim(commands, obstacles);
    println!("Result: {}", result);
}
