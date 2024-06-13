struct Solution(Vec<i32>, Vec<i32>);

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort();
        students.sort();

        seats.iter()
            .zip(students.iter())
            .map(|(seat, student)| (seat - student).abs())
            .sum()
    }
}

fn main() {
    println!("Hello, world!");
    let seats = vec![3, 1, 5];
    let students = vec![2, 7, 4];
    let result = Solution::min_moves_to_seat(seats, students);
    println!("Result: {}", result);
}
