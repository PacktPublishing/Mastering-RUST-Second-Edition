// thread_moves.rs

use std::thread;

fn main() {
    let my_str = String::from("Damn you borrow checker!");
    let _ = thread::spawn(move || {
        println!("In thread: {}", my_str);
    });
    println!("In main: {}", my_str);
}
