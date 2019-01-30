// string_slices_func.rs

fn say_hello(to_whom: &str) { 
    println!("Hey {}!", to_whom) 
} 

fn main() { 
    let string_slice: &'static str = "you"; 
    let string: String = string_slice.into(); 
    say_hello(string_slice); 
    say_hello(&string); 
}
