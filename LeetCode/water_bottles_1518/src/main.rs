struct Solution (i32, i32);

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total_drunk = num_bottles;
        let mut empty_bottles = num_bottles;

        while empty_bottles >= num_exchange {
            let new_bottles = empty_bottles / num_exchange;
            total_drunk += new_bottles;
            empty_bottles = empty_bottles % num_exchange + new_bottles;
        }

        total_drunk
    }
}


fn main() {
    println!("Hello, world!");

    let num_bottles = 9;
    let num_exchange = 3;
    let result = Solution::num_water_bottles(num_bottles, num_exchange);
    println!("Result: {}", result);
}
