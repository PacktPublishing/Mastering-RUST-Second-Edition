
use std::char::from_u32; 

fn main() { 
    let heart = from_u32(0x2764).unwrap();
    println!("I {} Rust!", heart); 
}