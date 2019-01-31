//  lifetime_struct.rs

struct Number<'a> { 
    num: &'a u8 
}

fn main() {
    let _n = Number {num: &545}; 
}
