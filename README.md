# Jellyfish-with-Rust

<i> "Rust is technology from the past came to save the future from itself." </i>

## Note:

- Cargo is the Rust package manager and build system. Cargo is part of the standard Rust toolset.

```bash

# Create package
cargo new <pack name>

# Build in debug mode
cargo build 

# Build in release mode
cargo build --release

cargo run

# Check infor
cargo check

```

- Keyword

```rust
pub enum Color { // pub => public, can import and use in an other file
    Red,
    Green,
    Blue
}


mod mytypes; // => load the module
use mytypes::Color; // => Intruduce the Color type into scope 

pub enum Color {
    #[allow(dead_code)] Red, // => attribute: allow Red not to be use
    #[allow(dead_code)] Green,
    #[allow(dead_code)] Blue
}

// Option Enums
enum Option<T> {
    Some(T),
    None
}

// Results Enums
enum Result<T, E> {
    Ok(T),
    Err(E)
}

fn sec_of_day(h: u32, m: u32, s: u32) -> Option<u32> { // => function with return type Option<u32>
    // some code here
}


// https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
use crate::mytypes::point::Point;

pub fn do_it() {

    println!("\nIn demo_modular_code::do_it()");

    let mut p1 = Point {x: 10, y: 20};

    p1.move_by(100, 200);
    println!("{}", p1.to_string());

    p1.reset();
    println!("{}", p1.to_string());
}

```

- <b> loop </b> : A keyword used for an infinite loop, which can be exited using a break statement.

- <b> while </b> : A conditional loop that continues as long as its condition is true.

- <b> for </b> : A loop that iterates through elements of a collection or range.

- <b> break </b> : A control flow keyword to exit the current innermost loop early.

- <b> mutability </b> : The ability for a variable to have its value changed during runtime by marking it as mutable with the mut keyword

- <b> option </b> : A Rust enum type that can be either Some(value) or None, used to represent optional values.

- <b> continue </b> : A control flow keyword to skip an iteration and move on to the next one in the same loop.

- <b> if let </b> : A pattern matching construct that allows you to conditionally bind variables based on their match against a given value or pattern.

- <b> sum </b> : An enum type wrapper around Option<T> which can be either Some(value) or None.

- <b> range </b> : Represents a sequence of numbers, often used in loops for iteration purposes.

- <b> shadowing </b> : A variable redeclaration with the same name but different value and/or scope within the same context.





