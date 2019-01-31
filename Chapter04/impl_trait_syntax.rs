// impl_trait_syntax.rs

use std::fmt::Display;

fn show_me(val: impl Display) {
    println!("{}", val);
}

fn main() {
    show_me("Trait bounds are awesome");
}
