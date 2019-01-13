// function_mut.rs

fn increase_by(mut val: u32, how_much: u32) {
    val += how_much;
    println!("You made {} points", val);
}

fn main() {
    let score = 2048;
    increase_by(score, 30);
}
