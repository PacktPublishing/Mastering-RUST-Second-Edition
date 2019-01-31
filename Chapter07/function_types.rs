// function_types.rs

fn add_two(a: u32, b: u32) -> u32 {
    a + b
}

fn main() {
    let my_func = add_two;
    let res = my_func(3, 4);
    println!("{:?}", res);
}
