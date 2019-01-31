// ownership_functions.rs

fn take_the_n(n: u8) { }

fn take_the_s(s: String) { }

fn main() { 
    let n = 5; 
    let s = String::from("string"); 

    take_the_n(n); 
    take_the_s(s); 

    println!("n is {}", n); 
    println!("s is {}", s); 
}
