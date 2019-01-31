// without_cell.rs
 
use std::cell::Cell; 

#[derive(Debug)]
struct Bag { 
    item: Box<u32>
} 

fn main() { 
    let mut bag = Cell::new(Bag { item: Box::new(1) }); 
    let hand1 = &mut bag;
    let hand2 = &mut bag;
    *hand1 = Cell::new(Bag {item: Box::new(2)});
    *hand2 = Cell::new(Bag {item: Box::new(2)});
}
