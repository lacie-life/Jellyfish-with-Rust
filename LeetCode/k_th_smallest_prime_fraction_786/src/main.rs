struct Solution(Vec<i32>, i32);

impl Solution {
    pub fn kth_smallest_prime_fraction(arr: Vec<i32>, k: i32) -> Vec<i32> {
        let mut lo = 0.0;
        let mut hi = 1.0;
        let mut numerator = 0;
        let mut denominator = 1;

        while hi - lo > 1e-9 {
            let mid = (lo + hi) / 2.0;
            let mut count = 0;
            let mut max_numerator = 0;
            let mut max_denominator = 1;

            let mut j = 1;
            for i in 0..arr.len() {
                while j < arr.len() && arr[i] as f64 / arr[j] as f64 > mid {
                    j += 1;
                }
                if j == arr.len() {
                    break;
                }
                count += (arr.len() - j) as i32;
                if arr[i] * max_denominator > arr[j] * max_numerator {
                    max_numerator = arr[i];
                    max_denominator = arr[j];
                }
            }

            if count == k {
                numerator = max_numerator;
                denominator = max_denominator;
                break;
            } else if count < k {
                lo = mid;
            } else {
                hi = mid;
            }
        }

        vec![numerator, denominator]
    }
}

fn main() {
    println!("Hello, world!");

    let arr = vec![1, 2, 3, 5];
    let k = 3;
    let result = Solution::kth_smallest_prime_fraction(arr, k);
    println!("{:?}", result);

    let arr = vec![1, 7];
    let k = 1;
    let result = Solution::kth_smallest_prime_fraction(arr, k);
    println!("{:?}", result);
}
