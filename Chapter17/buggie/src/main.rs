// buggie/src/main.rs

use std::env;

pub fn fibonacci(n: u32) -> u32 {
    let mut a = 0;
    let mut b = 1;
    let mut c = 0;
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    c
}

fn main() {
    let arg = env::args().skip(1).next().unwrap();
    let pos = str::parse::<u32>(&arg).unwrap();
    let nth_fib = fibonacci(pos);
    println!("Fibonacci number at {} is {}", pos, nth_fib);
}
