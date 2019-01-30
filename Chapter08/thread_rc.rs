// thread_rc.rs

use std::thread;
use std::rc::Rc;

fn main() {
    let nums = Rc::new(vec![0, 1, 2, 3, 4]);
    let mut childs = vec![];
    for n in 0..5 {
        let ns = nums.clone();
        let c = thread::spawn(|| {
            println!("{}", ns[n]);
        });
        childs.push(c);
    }

    for c in childs {
        c.join().unwrap();
    }
}
