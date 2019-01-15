// print_formatting.rs

use std::collections::HashMap;

fn main() {
    let a = 3669732608;
    println!("{:p}", a);
    println!("{:x}", a);

    // pretty printing
    let mut map = HashMap::new();
    map.insert("foo", "bar");
    println!("{:#?}", map);
}
