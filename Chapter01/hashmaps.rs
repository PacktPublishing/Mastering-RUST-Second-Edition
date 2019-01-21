// hashmaps.rs

use std::collections::HashMap; 

fn main() { 
    let mut fruits = HashMap::new(); 
    fruits.insert("apple", 3);
    fruits.insert("mango", 6);
    fruits.insert("orange", 2);
    fruits.insert("avocado", 7);
    for (k, v) in &fruits {
        println!("I got {} {}", v, k);
    }

    fruits.remove("orange");
    let old_avocado = fruits["avocado"];
    fruits.insert("avocado", old_avocado + 5);
    println!("\nI now have {} avocados", fruits["avocado"]);
}
