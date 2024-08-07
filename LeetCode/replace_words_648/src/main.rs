struct Solution (Vec<String>, String);

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut dictionary = dictionary;
        dictionary.sort_by(|a, b|a.len().cmp(&b.len()));
        let mut ret_vec = vec![];
        'outer: for word in sentence.split_ascii_whitespace() {
            for prefix in &dictionary {
                if word.starts_with(prefix) {
                    ret_vec.push(prefix.clone());
                    continue 'outer;
                }
            }
            ret_vec.push(word.to_string())
        }
        ret_vec.join(" ")
    }
}

fn main() {
    println!("Hello, world!");

    let dictionary = vec!["cat".to_string(), "bat".to_string(), "rat".to_string()];
    let sentence = "the cattle was rattled by the battery".to_string();
    let result = Solution::replace_words(dictionary, sentence);
    println!("result = {}", result);
}
