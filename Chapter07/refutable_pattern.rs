// refutable_pattern.rs

enum Container {
    Item(u64),
    Empty
}

fn main() {
    let mut item = Container::Item(56);
    let Container::Item(it) = item;
}
