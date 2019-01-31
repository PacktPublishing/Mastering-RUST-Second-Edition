// explicit_copy.rs

#[derive(Clone, Debug)]
struct Dummy {
    items: u32
}

fn main() {
    let a = Dummy { items: 54 };
    let b = a.clone();
    println!("a: {:?}, b: {:?}", a, b);
}
