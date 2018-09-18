// trait_objects.rs
 
use std::fmt::Debug;

#[derive(Debug)] 
struct Point { 
    x: i8, 
    y: i8 
} 

#[derive(Debug)] 
struct ThreeDimPoint { 
    x: i8, 
    y: i8, 
    z: i8 
}

fn main() { 
    let point = Point { x: 1, y: 3}; 
    let three_d_point = ThreeDimPoint { x: 3, y: 5, z: 9 }; 

    let mut x: &Debug = &point as &Debug;
    println!("1: {:?}", x); 

    x = &three_d_point; 
    println!("2: {:?}", x); 
}
