// ownership_closures.rs

#[derive(Debug)]
struct Foo;

fn main() {
    let a = Foo;

    let closure = || {
        let b = a;    
    };
    
    println!("{:?}", a);
}
