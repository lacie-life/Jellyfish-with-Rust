struct Solution;

impl Solution {
    pub fn sort_array(mut nums: Vec<i32>) -> Vec<i32> {
        fn mergesort(nums: &mut [i32]) {
            if nums.len() <= 1 {
                return;
            }
            let mid = nums.len() / 2;
            mergesort(&mut nums[..mid]);
            mergesort(&mut nums[mid..]);
            merge(nums, mid);
        }

        fn merge(nums: &mut [i32], mid: usize) {
            let mut left = nums[..mid].to_vec();
            let mut right = nums[mid..].to_vec();
            let (mut i, mut j, mut k) = (0, 0, 0);

            while i < left.len() && j < right.len() {
                if left[i] <= right[j] {
                    nums[k] = left[i];
                    i += 1;
                } else {
                    nums[k] = right[j];
                    j += 1;
                }
                k += 1;
            }

            while i < left.len() {
                nums[k] = left[i];
                i += 1;
                k += 1;
            }

            while j < right.len() {
                nums[k] = right[j];
                j += 1;
                k += 1;
            }
        }

        mergesort(&mut nums);
        nums
    }
}

fn main() {
    println!("Hello, world!");

    let nums = vec![5, 2, 3, 1];
    let result = Solution::sort_array(nums);
    println!("{:?}", result);
}
