struct Robot {
    index: usize,
    position: i32,
    health: i32,
    direction: char,
}

struct Solution(Vec<i32>, Vec<i32>, String);

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let mut robots: Vec<Robot> = positions
            .iter()
            .zip(healths.iter())
            .zip(directions.chars())
            .enumerate()
            .map(|(i, ((&position, &health), direction))| Robot {
                index: i,
                position,
                health,
                direction,
            })
            .collect();

        robots.sort_by_key(|robot| robot.position);

        let mut stack: Vec<Robot> = Vec::new();

        for mut robot in robots {
            if robot.direction == 'R' {
                stack.push(robot);
                continue;
            }

            while let Some(last) = stack.last_mut() {
                if last.direction == 'R' && robot.health > 0 {
                    if last.health == robot.health {
                        stack.pop();
                        robot.health = 0;
                    } else if last.health < robot.health {
                        stack.pop();
                        robot.health -= 1;
                    } else {
                        last.health -= 1;
                        robot.health = 0;
                    }
                } else {
                    break;
                }
            }

            if robot.health > 0 {
                stack.push(robot);
            }
        }

        stack.sort_by_key(|robot| robot.index);

        stack.into_iter().map(|robot| robot.health).collect()
    }
}


fn main() {
    println!("Hello, world!");

    let positions = vec![5, 4, 3, 2, 1];
    let healths = vec![2, 17, 9, 15, 10];
    let directions = "RRRRR".to_string();
    let result = Solution::survived_robots_healths(positions, healths, directions);
    println!("{:?}", result);

    let positions = vec![3, 5, 2, 6];
    let healths = vec![10, 10, 15, 12];
    let directions = "RLRL".to_string();
    let result = Solution::survived_robots_healths(positions, healths, directions);
    println!("{:?}", result);

}
