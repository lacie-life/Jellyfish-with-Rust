struct Solution;

impl Solution {
    pub fn number_to_words(num: i32) -> String {
        if num == 0 {
            return "Zero".to_string();
        }
        Solution::helper(num)
    }

    fn helper(num: i32) -> String {
        const BELOW_TWENTY: [&str; 20] = [
            "", "One", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten",
            "Eleven", "Twelve", "Thirteen", "Fourteen", "Fifteen", "Sixteen", "Seventeen", "Eighteen", "Nineteen"
        ];
        const TENS: [&str; 10] = [
            "", "", "Twenty", "Thirty", "Forty", "Fifty", "Sixty", "Seventy", "Eighty", "Ninety"
        ];

        let s = if num < 20 {
            BELOW_TWENTY[num as usize].to_string()
        } else if num < 100 {
            format!("{} {}", TENS[(num / 10) as usize], BELOW_TWENTY[(num % 10) as usize])
        } else if num < 1000 {
            format!("{} Hundred {}", Solution::helper(num / 100), Solution::helper(num % 100))
        } else if num < 1_000_000 {
            format!("{} Thousand {}", Solution::helper(num / 1000), Solution::helper(num % 1000))
        } else if num < 1_000_000_000 {
            format!("{} Million {}", Solution::helper(num / 1_000_000), Solution::helper(num % 1_000_000))
        } else {
            format!("{} Billion {}", Solution::helper(num / 1_000_000_000), Solution::helper(num % 1_000_000_000))
        };

        Solution::trim(s)
    }

    fn trim(s: String) -> String {
        s.trim().to_string()
    }
}

fn main() {
    println!("Hello, world!");
    let num = 1234567891;
    let result = Solution::number_to_words(num);
    println!("The number {} in words is: {}", num, result);
}
