use std::env;

fn main() {
    let name = env::args().skip(1).next();

    match name {
        Some(n) => println!("Hi ! {}", n), 
        None => panic!("Didn't receive any name ?")
    }
}


