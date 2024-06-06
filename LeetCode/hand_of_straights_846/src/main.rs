use std::collections::BTreeMap;

struct Solution;

impl Solution {
    pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
        let group_size = group_size as usize;
        let mut count = BTreeMap::new();

        // Count the frequency of each card
        for &card in &hand {
            *count.entry(card).or_insert(0) += 1;
        }

        // Try to form groups
        let mut keys: Vec<i32> = count.keys().cloned().collect();
        for &start in &keys {
            let value = *count.get(&start).unwrap();
            if value > 0 {
                for i in 0..group_size {
                    let current_card = start + i as i32;
                    if let Some(current_count) = count.get_mut(&current_card) {
                        if *current_count < value {
                            return false;
                        }
                        *current_count -= value;
                    } else {
                        return false;
                    }
                }
            }
        }

        true
    }
}

fn main() {
    println!("Hello, world!");
    let hand = vec![1, 2, 3, 6, 2, 3, 4, 7, 8];
    let group_size = 3;
    let result = Solution::is_n_straight_hand(hand, group_size);
    println!("Can the hand be rearranged into groups of {}? {}", group_size, result);
}
