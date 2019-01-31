// trait_bounds_basics.rs

fn add_thing<T>(fst: T, snd: T) {
    let _ = fst + snd;
}

fn main() {
    add_thing(2, 2);
}
