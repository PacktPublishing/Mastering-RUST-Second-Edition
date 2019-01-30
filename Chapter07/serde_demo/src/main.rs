
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Foo {
    a: String,
    b: u64
}

impl Foo {
    fn new(a: &str, b: u64) -> Self {
        Self {
            a: a.to_string(),
            b
        }
    }
}

fn main() {
    let foo_json = serde_json::to_string(&Foo::new("It's that simple", 101)).unwrap();
    println!("{:?}", foo_json);
    let foo_value: Foo = serde_json::from_str(&foo_json).unwrap();
    println!("{:?}", foo_value);
}
