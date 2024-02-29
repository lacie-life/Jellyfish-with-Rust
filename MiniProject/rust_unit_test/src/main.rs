// This example demonstrates how to write unit tests in Rust.


fn main() {
    let result = add_numbers(2, 3);
    println!("Result: {}", result);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_numbers() {
        assert_eq!(add_numbers(2, 3), 5);
        assert_eq!(add_numbers(0, 0), 0);
        assert_eq!(add_numbers(-1, 1), 0);
        assert_eq!(add_numbers(-1, -1), -2);
    }
}


