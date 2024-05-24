use std::collections::HashMap;

struct Solution(Vec<String>, Vec<char>, Vec<i32>);

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        let words = words
            .into_iter()
            .map(|str| str.bytes().map(|b| (b - b'a') as usize).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let values = words
            .iter()
            .map(|idxes| idxes.iter().map(|idx| score[*idx]).sum::<i32>())
            .collect::<Vec<_>>();
        let letters = letters.into_iter().fold(vec![0; 26], |mut vec, letter| {
            vec[(letter as u8 - b'a') as usize] += 1;
            vec
        });

        let mut map = HashMap::new();
        map.insert([0; 26], 0);
        let mut max = 0;
        for (idx, word) in words.into_iter().enumerate() {
            let count = word.into_iter().fold([0; 26], |mut arr, idx| {
                arr[idx] += 1;
                arr
            });
            for (mut k, v) in map.clone() {
                k.iter_mut().zip(count.iter()).for_each(|(k, c)| *k += c);
                if k.iter()
                    .enumerate()
                    .all(|(idx, letter)| *letter <= letters[idx])
                {
                    max = max.max(v + values[idx]);
                    map.insert(k, v + values[idx]);
                }
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");

    let words = vec!["dog".to_string(), "cat".to_string(), "dad".to_string(), "good".to_string()];
    let letters = vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'];
    let score = vec![1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let result = Solution::max_score_words(words.into_iter().map(|s| s.to_string()).collect(), letters, score);
    println!("result = {:?}", result);
}
