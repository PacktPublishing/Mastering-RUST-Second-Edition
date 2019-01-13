// tuples.rs

fn main() { 
    let num_and_str: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", num_and_str);
    let (num, string) = num_and_str;
    println!("From tuple: Number: {}, String: {}", num, string);
}
