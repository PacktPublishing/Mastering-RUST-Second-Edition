
// shared_owner_threads.rs

use std::thread;

struct Payload {
    data: Rc<u32>
}

fn main() {
    let p = Payload {
        data: Rc::new(0)
    };
    for i in 0..10 {

    }
}
