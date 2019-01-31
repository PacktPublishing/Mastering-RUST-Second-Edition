// enum_struct_consts.rs

use std::collections::HashMap;

#[derive(Debug)]
struct Inventory {
    inner: HashMap<String, u32>
}

impl Inventory {
    const DEFAULT_COUNT: u32 = 20;
}

fn put_into_inventory(inventory: &mut Inventory, item: String, count: Option<u32>) {
    if let Some(c) = count {
        inventory.inner.insert(item, c);
    } else {
        inventory.inner.insert(item, Inventory::DEFAULT_COUNT);
    }
}

fn main() {
    let food = "Apple".to_string();
    let mut inventory = Inventory { inner: HashMap::new() };
    put_into_inventory(&mut inventory, food, None);
    println!("{:?}", inventory);
}
