// my_program/main.rs

mod foo;

use foo::Bar;

fn main() {
    foo::do_foo();
    Bar::hello();
}
