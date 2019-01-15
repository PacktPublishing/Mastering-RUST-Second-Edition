// first_macro.rs
     
use std::io::stdin;
     
// A convenient macro to read input as string into a buffer
macro_rules! scanline {
    ($x:expr) => ({
        stdin().read_line(&mut $x).unwrap();
        $x.trim();
    });
    () => ({
        let mut s = String::new();
        stdin().read_line(&mut s).unwrap();
        s
    });
}

fn main() {
    let mut input = String::new();
    scanline!(input);
    println!("Hi {}",input);
    let a = scanline!();
    println!("Hi {}", a);
}
