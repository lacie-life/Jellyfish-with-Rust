struct Solution (Vec<Vec<i32>>);

impl Solution {
    pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
        let mut wait: f64 = 0.0;
        let mut curr: f64 = 0.0;

        for c in customers.iter() {
            curr = curr.max(c[0] as f64) + c[1] as f64;
            wait += curr - c[0] as f64;
        }

        wait / customers.len() as f64
    }
}


fn main() {
    println!("Hello, world!");

    let customers = vec![vec![1, 2], vec![2, 5], vec![4, 3]];
    let result = Solution::average_waiting_time(customers);
    println!("Average waiting time: {}", result);

    let customers = vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]];
    let result = Solution::average_waiting_time(customers);
    println!("Average waiting time: {}", result);
}
