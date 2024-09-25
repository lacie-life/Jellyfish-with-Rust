use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TrieNode {
    children: Vec<Option<Rc<RefCell<TrieNode>>>>,
    count: i32,
}

impl TrieNode {
    fn new() -> Rc<RefCell<TrieNode>> {
        Rc::new(RefCell::new(TrieNode {
            children: vec![None; 26], // For each letter 'a' to 'z'
            count: 0,
        }))
    }
}

struct Solution;

impl Solution {
    pub fn sum_prefix_scores(words: Vec<String>) -> Vec<i32> {
        let root = TrieNode::new();
        let mut ans = Vec::new();

        // Insert each word into the Trie
        for word in &words {
            Self::insert(&root, word);
        }

        // Calculate the score for each word
        for word in &words {
            ans.push(Self::get_score(&root, word));
        }

        ans
    }

    fn insert(root: &Rc<RefCell<TrieNode>>, word: &str) {
        let mut node = Rc::clone(root);
        for c in word.chars() {
            let idx = (c as usize) - ('a' as usize);
            let mut child = node.borrow_mut().children[idx].take();
            if child.is_none() {
                child = Some(TrieNode::new());
            }
            let next_node = Rc::clone(child.as_ref().unwrap());
            next_node.borrow_mut().count += 1;
            node.borrow_mut().children[idx] = child;
            node = next_node;
        }
    }

    fn get_score(root: &Rc<RefCell<TrieNode>>, word: &str) -> i32 {
        let mut node = Rc::clone(root);
        let mut score = 0;
        for c in word.chars() {
            let idx = (c as usize) - ('a' as usize);
            let next_node = Rc::clone(node.borrow().children[idx].as_ref().unwrap());
            score += next_node.borrow().count;
            node = next_node;
        }
        score
    }
}

fn main() {
    println!("Hello, world!");

    let words = vec!["abc", "ab", "bc", "b"];
    let result = Solution::sum_prefix_scores(words.iter().map(|s| s.to_string()).collect());
    println!("{:?}", result);
}