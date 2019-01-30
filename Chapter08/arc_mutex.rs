// arc_mutex.rs

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let vec = Arc::new(Mutex::new(vec![]));
    let mut childs = vec![];
    for i in 0..5 {
        let mut v = vec.clone();
        let t = thread::spawn(move || {
            let mut v = v.lock().unwrap(); 
            v.push(i);
        });
        childs.push(t);
    }

    for c in childs {
        c.join().unwrap();
    }

    println!("{:?}", vec);
}
