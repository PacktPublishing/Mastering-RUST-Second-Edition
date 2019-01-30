// mutex_basics.rs

use std::sync::Mutex;
use std::thread;

fn main() {
    let m = Mutex::new(0);
    let c = thread::spawn(move || {
        {
            *m.lock().unwrap() += 1;
        }
        let updated = *m.lock().unwrap();
        updated
    });
    let updated = c.join().unwrap();
    println!("{:?}", updated);
}
