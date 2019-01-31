// trait_bound_basics_fixed.rs

use std::ops::Add;

fn add_thing<T: Add>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    add_thing(2, 2);
}
