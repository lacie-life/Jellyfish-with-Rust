struct Solution (Vec<i32>, i32);

impl Solution {
    pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
        let mut people = people;
        people.sort();
        let mut i = 0;
        let mut j = people.len() - 1;
        let mut boats = 0;
        while i <= j {
            if people[i] + people[j] <= limit {
                i += 1;
            }
            println!("before i: {}, j: {}", i, j);

            boats += 1;

            if j == 0 {
                break;
            } else {
                j -= 1;
            }

            println!("after i: {}, j: {}", i, j);


        }
        boats
    }
}

fn main() {
    println!("Hello, world!");

    let people = vec![3, 2, 2, 1];
    let limit = 3;
    let result = Solution::num_rescue_boats(people, limit);
    println!("Result: {}", result);

    let people = vec![3, 5, 3, 4];
    let limit = 5;
    let result = Solution::num_rescue_boats(people, limit);
    println!("Result: {}", result);
}
