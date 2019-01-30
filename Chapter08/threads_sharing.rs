
// shared_owner_threads.rs

use std::thread;
use std::rc::Rc;
use std::sync::Mutex;

struct Payload {
    data: Mutex<u32>
}

impl Payload {
    fn new(val: u32) -> Self {
        Payload {
            data: Mutex::new(val)
        }
    }

    fn inc(&mut self) {
        *self.data.lock().unwrap() += 1;
    }
}

fn main() {
    let a = Rc::new(Payload::new(0));
    thread::spawn(|| {
        loop {
            let b = a.clone();
        }
    });
}

