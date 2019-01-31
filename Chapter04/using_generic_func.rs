// using_generic_func.rs

use std::str;

fn main() {
    let num_from_str = str::parse::<u8>("34").unwrap();
    println!("Parsed number {}", num_from_str);
}
