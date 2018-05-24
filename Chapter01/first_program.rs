
// first_program.rs

use std::env;

fn main() {
    let name = env::args().skip(1).next();
    match name {
        Some(name) => println!("Hi there ! {}", name),
        None => panic!("Didn't receive any name ?")
    }
}
