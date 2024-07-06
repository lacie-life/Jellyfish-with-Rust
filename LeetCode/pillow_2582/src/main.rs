struct Solution(i32, i32);

impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let cycle_length = (n - 1) * 2;
        let time = time % cycle_length;

        if time < n {
            1 + time
        } else {
            n - (time - (n - 1))
        }
    }
}

fn main() {
    println!("Hello, world!");
}
