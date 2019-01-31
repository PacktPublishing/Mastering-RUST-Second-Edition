// return_func_ref.rs

fn get_a_borrowed_value() -> &u8 { 
    let x = 1; 
    &x 
}

fn main() {
    let value = get_a_borrowed_value(); 
}
