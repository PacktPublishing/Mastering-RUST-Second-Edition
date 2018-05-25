// variables.rs

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}
 
fn main() {
    let mut target = "world";
    let mut greeting = "Hello";
    println!("{}, {}", greeting, target);
    greeting = "How are you doing";
    target = "mate";
    println!("{}, {}", greeting, target);

    let score = 2048;
    increase_by(score, 30);
}
