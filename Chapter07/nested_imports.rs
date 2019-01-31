// nested_imports.rs

use std::sync::{Arc, Mutex, mpsc::{channel, Sender, Receiver}};

fn consume(_tx: Sender<()>, _rx: Receiver<()>) { } 

fn main() {
    let _ = Arc::new(Mutex::new(40));
    let (tx, rx) = channel();
    consume(tx, rx);
}
