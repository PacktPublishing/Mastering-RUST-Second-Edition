// generic_struct_impl.rs

struct Container<T> {
    item: T
}

impl<T> Container<T> {
    fn new(item: T) -> Self {
        Container { item }
    }
}

fn main() {
    // stuff
}
