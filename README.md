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

```





