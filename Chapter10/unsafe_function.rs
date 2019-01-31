// unsafe_function.rs

fn get_value(i: *const i32) -> i32 { 
    unsafe {*i}
}

fn main() {
    let foo = 4 as *const i32;
    let _bar = get_value(foo);
}
