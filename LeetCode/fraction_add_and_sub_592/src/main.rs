struct Solution;

impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        let fractions = Solution::parse_expression(&expression);
        let (numerator, denominator) = Solution::calculate_result(fractions);
        Solution::format_fraction(numerator, denominator)
    }

    fn parse_expression(expression: &str) -> Vec<(i32, i32)> {
        let mut fractions = Vec::new();
        let mut num = String::new();
        let mut denom = String::new();
        let mut is_numerator = true;
        let mut sign = 1;

        for c in expression.chars() {
            match c {
                '+' | '-' => {
                    if !num.is_empty() && !denom.is_empty() {
                        let numerator: i32 = num.parse().unwrap();
                        let denominator: i32 = denom.parse().unwrap();
                        fractions.push((sign * numerator, denominator));
                        num.clear();
                        denom.clear();
                    }
                    sign = if c == '+' { 1 } else { -1 };
                    is_numerator = true;
                }
                '/' => {
                    is_numerator = false;
                }
                _ => {
                    if is_numerator {
                        num.push(c);
                    } else {
                        denom.push(c);
                    }
                }
            }
        }

        if !num.is_empty() && !denom.is_empty() {
            let numerator: i32 = num.parse().unwrap();
            let denominator: i32 = denom.parse().unwrap();
            fractions.push((sign * numerator, denominator));
        }

        fractions
    }

    fn calculate_result(fractions: Vec<(i32, i32)>) -> (i32, i32) {
        let mut numerator = 0;
        let mut denominator = 1;

        for (num, denom) in fractions {
            numerator = numerator * denom + num * denominator;
            denominator *= denom;
            let gcd = Solution::gcd(numerator.abs(), denominator);
            numerator /= gcd;
            denominator /= gcd;
        }

        (numerator, denominator)
    }

    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 {
            a
        } else {
            Solution::gcd(b, a % b)
        }
    }

    fn format_fraction(numerator: i32, denominator: i32) -> String {
        if denominator == 1 {
            format!("{}/1", numerator)
        } else {
            format!("{}/{}", numerator, denominator)
        }
    }
}

fn main() {
    println!("Hello, world!");

    let expression = "-1/2+1/2".to_string();
    let result = Solution::fraction_addition(expression);
    println!("Result: {}", result);
}
