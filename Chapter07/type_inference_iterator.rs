// type_inference_iterator.rs

use std::fs::File;
use std::io::Read;

fn main() {
    let file = File::open("foo.txt").unwrap();
    let bytes = file.bytes().collect();
}
