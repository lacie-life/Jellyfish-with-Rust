pub struct Solution;

impl Solution {
    pub fn count_triplets(arr: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut xors = vec![0];
        let mut prefix = 0;

        // Calculate prefix XORs
        for &a in &arr {
            prefix ^= a;
            xors.push(prefix);
        }

        // Count valid triplets
        let n = arr.len();
        for j in 1..n {
            for i in 0..j {
                let xors_i = xors[j] ^ xors[i];
                for k in j..n {
                    let xors_k = xors[k + 1] ^ xors[j];
                    if xors_i == xors_k {
                        ans += 1;
                    }
                }
            }
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");

    let arr = vec![2, 3, 1, 6, 7];
    let result = Solution::count_triplets(arr);
    println!("Number of triplets: {}", result);
}
