// into_map_demo/src/main.rs

use into_map_derive::IntoMap;

#[derive(IntoMap)]
struct User {
    name: String,
    id: usize,
    active: bool
}

fn main() {
    let my_bar = User { name: "Alice".to_string(), id: 35, active: false };
    let map = my_bar.into_map();
    println!("{:?}", map);
}
