struct Solution;

impl Solution {
    pub fn xor_queries(arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut xors = vec![0; arr.len() + 1];

        // Precompute the XOR prefix for the array
        for (i, &num) in arr.iter().enumerate() {
            xors[i + 1] = xors[i] ^ num;
        }

        // Process each query
        for query in queries {
            let left = query[0] as usize;
            let right = query[1] as usize;
            ans.push(xors[left] ^ xors[right + 1]);
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");

    let arr = vec![1, 3, 4, 8];
    let queries = vec![vec![0, 1], vec![1, 2], vec![0, 3], vec![3, 3]];
    let result = Solution::xor_queries(arr, queries);
    println!("{:?}", result);
}
