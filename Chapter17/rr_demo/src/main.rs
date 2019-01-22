
use rand::prelude::*;

use std::thread;
use std::time::Duration;

fn main() {
    for i in 0..10 {
        thread::spawn(move || {
            let mut rng = rand::thread_rng();
            let y: f64 = rng.gen();
            let a: u64 = rand::thread_rng().gen_range(0, 100); 
            thread::sleep(Duration::from_millis(a));
            print!("{} ", i);
        });
    }
    thread::sleep(Duration::from_millis(1000));
    println!("Hello, world!");
}
