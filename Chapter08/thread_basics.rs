// thread_basics.rs

use std::thread;

fn main() {
    thread::spawn(|| {
        println!("Thread!");
        "Much concurrent, such wow!".to_string()
    });
    print!("Hello ");
}
