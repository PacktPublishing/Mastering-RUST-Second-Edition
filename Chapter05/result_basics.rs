// result_basics.rs

use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let path = Path::new("data.txt");
    let file = File::open(&path);
    let mut s = String::new();
    file.read_to_string(&mut s);
    println!("Message: {}", s);
}
