// string_range_slice.rs

fn main() {
    let hello = String::from("Strings are cool");
    let first_char = &hello[0..3];
    println!("{:?}", first_char);
}
