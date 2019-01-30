// thread_rwlock.rs

use std::sync::RwLock;
use std::thread;

fn main() {
    let m = RwLock::new(5);
    let c = thread::spawn(move || {
        {
            *m.write().unwrap() += 1;
        }
        let updated = *m.read().unwrap();
        updated
    });
    let updated = c.join().unwrap();
    println!("{:?}", updated);
}
