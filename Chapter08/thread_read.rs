// thread_read.rs

use std::thread;

fn main() {
    let nums = vec![0, 1, 2, 3, 4];
    for n in 0..5 {
        thread::spawn(|| {
            println!("{}", nums[n]);
        });
    }
}
