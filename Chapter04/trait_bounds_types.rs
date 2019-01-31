
// trait_bounds_types.rs

use std::fmt::Display;

struct Foo<T: Display> {
    bar: T
}

// or

struct Bar<F> where F: Display {
    inner: F
}

fn main() {}
