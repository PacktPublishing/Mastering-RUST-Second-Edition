// ownership_basics.rs

#[derive(Debug)]
struct Foo;

fn main() {
    let foo = Foo;
    let bar = &foo;
    println!("Foo is {:?}", foo);
    println!("Bar is {:?}", bar);
}
