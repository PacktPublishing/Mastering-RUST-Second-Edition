// trait_objects.rs

use std::fmt::Debug;

#[derive(Debug)]
struct Square(f32);
#[derive(Debug)]
struct Rectangle(f32, f32);

trait Area: Debug {
    fn get_area(&self) -> f32; 
}

impl Area for Square {
    fn get_area(&self) -> f32 {
        self.0 * self.0
    }
}

impl Area for Rectangle {
    fn get_area(&self) -> f32 {
        self.0 * self.1
    }
}

fn main() {
    let shapes: Vec<&dyn Area> = vec![&Square(3f32), &Rectangle(4f32, 2f32)];
    for s in shapes {
        println!("{:?}", s);
    }
}
