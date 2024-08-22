struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let bit_length = 32 - num.leading_zeros();
        let mask = (1 << bit_length) - 1;
        num ^ mask
    }
}

fn main() {
    println!("Hello, world!");

    let num = 5;
    let result = Solution::find_complement(num);
    println!("The complement of {} is {}", num, result);
}
