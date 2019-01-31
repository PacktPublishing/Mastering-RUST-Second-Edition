// using_lifetimes.rs

struct SomeRef<'a, T> {
    part: &'a T
}

fn main() {
    let _a = SomeRef { part: &43 };
}
