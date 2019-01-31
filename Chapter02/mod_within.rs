// mod_within.rs

mod food {
    struct Cake;
    struct Smoothie;
    struct Pizza;
}

use food::Cake;

fn main() {
    let eatable = Cake;
}
