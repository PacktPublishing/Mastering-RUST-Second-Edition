// using_generic_vec.rs

fn main() {
    // providing a type
    let v1: Vec<u8> = Vec::new();

    // or calling method
    let mut v2 = Vec::new();
    v2.push(2);    // v2 is now Vec<i32>

    // or using turbofish
    let v3 = Vec::<u8>::new();    // not so readable
}
