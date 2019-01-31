// trait_bounds_functions.rs

use std::fmt::Debug;

trait Eatable {
    fn eat(&self);
}

#[derive(Debug)]
struct Food<T>(T);

#[derive(Debug)]
struct Apple;

impl<T> Eatable for Food<T> where T: Debug {
    fn eat(&self) {
        println!("Eating {:?}", self);
    }
}

fn eat<T>(val: T) where T: Eatable {
    val.eat();
}

fn main() {
    let apple = Food(Apple);
    eat(apple);
}
