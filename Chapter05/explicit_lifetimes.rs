// explicit_lifetimes.rs

fn foo(a: &str, b: &str) -> &str {
    b
}

fn main() {
    let a = "Hello";
    let b = "World";
    let c = foo(a, b);
}

