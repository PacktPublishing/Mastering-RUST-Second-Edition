// impl_trait_both.rs

use std::fmt::Display;

fn surround_with_braces(val: impl Display) -> impl Display {
    format!("{{{}}}", val)
}

fn main() {
    println!("{}", surround_with_braces("Hello"));
}
