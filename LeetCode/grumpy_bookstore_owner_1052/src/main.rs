struct Solution (Vec<i32>, Vec<i32>, i32);

impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, x: i32) -> i32 {
        let mut total_satisfied = 0;
        let mut max_convertible = 0;
        let mut window_convertible = 0;
        let x = x as usize;

        for i in 0..customers.len() {
            if grumpy[i] == 0 { total_satisfied += customers[i]; }
            else { window_convertible += customers[i]; }

            if i >= x {
                window_convertible -= if grumpy[i - x] == 1 { customers[i - x] } else { 0 };
            }

            max_convertible = max_convertible.max(window_convertible);
        }

        total_satisfied + max_convertible
    }
}


fn main() {
    println!("Hello, world!");

    let customers = vec![1,0,1,2,1,1,7,5];
    let grumpy = vec![0,1,0,1,0,1,0,1];
    let x = 3;
    let result = Solution::max_satisfied(customers, grumpy, x);
    println!("Result: {}", result);
}
