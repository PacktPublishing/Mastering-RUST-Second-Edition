// vec.rs

fn main() {
    let mut numbers_vec: Vec<u8> = Vec::new(); 
    numbers_vec.push(1); 
    numbers_vec.push(2); 

    let mut vec_with_macro = vec![1]; 
    vec_with_macro.push(2);
    let _ = vec_with_macro.pop();    // value ignored with `_`

    let message = if numbers_vec == vec_with_macro {
        "They are equal"
    } else {
        "Nah! They look different to me"
    };

    println!("{} {:?} {:?}", message, numbers_vec, vec_with_macro); 
}
