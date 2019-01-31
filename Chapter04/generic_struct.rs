// generic_struct.rs

struct Container<T> {
    item: T
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

impl Container<u32> {
    fn sum(item: u32) -> Self {
        Container { item }
    }
}

fn main() {
    // todo
}