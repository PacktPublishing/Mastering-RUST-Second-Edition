
fn main() { 
    let number_and_string: (u8, &str) = (40, "Have a good day!");
    println!("{:?}", number_and_string);

    let (num, string) = number_and_string;
    println!("From tuple: Number: {}, String: {}", num, string);
}