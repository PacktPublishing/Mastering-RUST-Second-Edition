// strings.rs

fn main() {
    let question = "How are you ?";            // a &str type
    let person: String = "Bob".to_string();
    let namaste = String::from("नमस्ते");        // unicodes yay!

    println!("{}! {} {}", namaste, question, person);
}
