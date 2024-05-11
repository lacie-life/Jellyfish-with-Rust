struct Solution(Vec<i32>, Vec<i32>, i32);

impl Solution {
    pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
        // Make workers as tuple of quality and wage
        let mut workers = quality.iter().zip(wage.iter()).collect::<Vec<_>>();

        // Sort workers by wage/quality ratio
        workers.sort_by(|a, b| {
            (*a.1 as f64 / *a.0 as f64).partial_cmp(&(*b.1 as f64 / *b.0 as f64)).unwrap()
        });

        // Use a max heap to keep the smallest k-1 quality
        let mut ans = std::f64::MAX;
        let mut sum = 0;
        let mut heap = std::collections::BinaryHeap::new();


        for (q, w) in workers.iter() {
            heap.push(*q); // Add quality of the current worker to heap
            sum += *q; // Add quality of the current worker to sum

            // If heap size is greater than k, remove the worker with the highest quality
            if heap.len() > k as usize {
                sum -= heap.pop().unwrap();
            }

            // If heap size is equal to k, calculate the cost and update the answer
            if heap.len() == k as usize {
                ans = ans.min(sum as f64 * (**w as f64 / **q as f64));
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");

    let quality = vec![10, 20, 5];
    let wage = vec![70, 50, 30];
    let k = 2;
    let result = Solution::mincost_to_hire_workers(quality, wage, k);
    println!("result = {}", result);
}
