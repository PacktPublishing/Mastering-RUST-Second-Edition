// for_loops.rs

fn main() {
    // does not include 10
    print!("Normal ranges: ");
    for i in 0..10 {
        print!("{},", i);
    }

    println!();       // just a newline
    print!("Inclusive ranges: ");
    // counts till 10
    for i in 0..=10 {
        print!("{},", i);
    }
}
