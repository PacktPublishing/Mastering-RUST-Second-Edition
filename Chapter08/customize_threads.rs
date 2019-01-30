// customize_threads.rs

use std::thread::Builder;

fn main() {
    let my_thread = Builder::new().name("Worker Thread".to_string())
                                  .stack_size(1024 * 4);
    let handle = my_thread.spawn(|| {
        panic!("Oops!");
    });
    let child_status = handle.unwrap().join();
    println!("Child status: {:?}", child_status);
}
