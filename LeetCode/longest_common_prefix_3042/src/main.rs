use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct TrieNode {
    children: Vec<Option<Rc<RefCell<TrieNode>>>>,
}

impl TrieNode {
    fn new() -> Rc<RefCell<TrieNode>> {
        Rc::new(RefCell::new(TrieNode {
            children: vec![None; 10], // We use 10 for digits 0-9.
        }))
    }
}

struct Trie {
    root: Rc<RefCell<TrieNode>>,
}

impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&self, word: &str) {
        let mut node = Rc::clone(&self.root);
        for c in word.chars() {
            let i = c as usize - '0' as usize;
            if node.borrow().children[i].is_none() {
                node.borrow_mut().children[i] = Some(TrieNode::new());
            }
            let next_node = Rc::clone(node.borrow().children[i].as_ref().unwrap());
            node = next_node;
        }
    }

    fn search(&self, word: &str) -> usize {
        let mut prefix_length = 0;
        let mut node = Rc::clone(&self.root);
        for c in word.chars() {
            let i = c as usize - '0' as usize;
            if node.borrow().children[i].is_none() {
                break;
            }
            let next_node = Rc::clone(node.borrow().children[i].as_ref().unwrap());
            drop(node); // Drop the current node to release the borrow
            node = next_node;
            prefix_length += 1;
        }
        prefix_length
    }
}

struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut ans = 0;
        let trie = Trie::new();

        for num in arr1 {
            trie.insert(&num.to_string());
        }

        for num in arr2 {
            ans = ans.max(trie.search(&num.to_string()) as i32);
        }

        ans
    }
}

fn main() {
    println!("Hello, world!");

    let arr1 = vec![1, 2, 3];
    let arr2 = vec![4, 4, 4];
    let result = Solution::longest_common_prefix(arr1, arr2);
    println!("Result: {}", result);
}