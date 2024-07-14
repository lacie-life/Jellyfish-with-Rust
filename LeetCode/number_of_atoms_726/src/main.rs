use std::collections::HashMap;

struct Solution(String);

impl Solution {
    pub fn count_of_atoms(formula: String) -> String {
        let mut i = 0;
        let mut count = Self::parse(&formula, &mut i);
        let mut elements: Vec<_> = count.into_iter().collect();
        elements.sort_by(|a, b| a.0.cmp(&b.0));

        let mut result = String::new();
        for (elem, freq) in elements {
            result.push_str(&elem);
            if freq > 1 {
                result.push_str(&freq.to_string());
            }
        }

        result
    }

    fn parse(s: &str, i: &mut usize) -> HashMap<String, i32> {
        let mut count = HashMap::new();

        while *i < s.len() {
            if s.chars().nth(*i) == Some('(') {
                *i += 1;
                let sub_count = Self::parse(s, i);
                let multiplier = Self::get_num(s, i);
                for (elem, freq) in sub_count {
                    *count.entry(elem).or_insert(0) += freq * multiplier;
                }
            } else if s.chars().nth(*i) == Some(')') {
                *i += 1;
                let multiplier = Self::get_num(s, i);
                for freq in count.values_mut() {
                    *freq *= multiplier;
                }
                return count;
            } else {
                let elem = Self::get_elem(s, i);
                let num = Self::get_num(s, i);
                *count.entry(elem).or_insert(0) += num;
            }
        }

        count
    }

    fn get_elem(s: &str, i: &mut usize) -> String {
        let start = *i;
        *i += 1;
        while *i < s.len() && s.chars().nth(*i).unwrap().is_lowercase() {
            *i += 1;
        }
        s[start..*i].to_string()
    }

    fn get_num(s: &str, i: &mut usize) -> i32 {
        let start = *i;
        while *i < s.len() && s.chars().nth(*i).unwrap().is_digit(10) {
            *i += 1;
        }
        if start == *i {
            1
        } else {
            s[start..*i].parse().unwrap()
        }
    }
}

fn main() {
    println!("Hello, world!");

    let formula = "H2O".to_string();
    let result = Solution::count_of_atoms(formula);
    println!("result = {}", result);

    let formula = "Mg(OH)2".to_string();
    let result = Solution::count_of_atoms(formula);
    println!("result = {}", result);

    let formula = "K4(ON(SO3)2)2".to_string();
    let result = Solution::count_of_atoms(formula);
    println!("result = {}", result);
}
